# Copyright 2021 The Chromium OS Authors. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

import argparse
import fnmatch
import functools
import itertools
import json
import os
import random
import subprocess
import sys
from multiprocessing import Pool
from pathlib import Path
from typing import Dict, Iterable, List, NamedTuple, Optional

from . import test_target, testvm
from .test_target import TestTarget, Triple
from .test_config import CRATE_OPTIONS, TestOption, BUILD_FEATURES

USAGE = """\
Runs tests for crosvm locally, in a vm or on a remote device.

To build and run all tests locally:

    $ ./tools/run_tests --target=host

To cross-compile tests for aarch64 and run them on a built-in VM:

    $ ./tools/run_tests --target=vm:aarch64

The VM will be automatically set up and booted. It will remain running between
test runs and can be managed with `./tools/aarch64vm`.

Tests can also be run on a remote device via SSH. However it is your
responsiblity that runtime dependencies of crosvm are provided.

    $ ./tools/run_tests --target=ssh:hostname

The default test target can be managed with `./tools/set_test_target`

To see full build and test output, add the `-v` or `--verbose` flag.
"""

# Print debug info. Overriden by -v
VERBOSE = False

# Timeouts for tests to prevent them from running too long.
TEST_TIMEOUT_SECS = 60
LARGE_TEST_TIMEOUT_SECS = 120

# Double the timeout if the test is running in an emulation environment, which will be
# significantly slower than native environments.
EMULATION_TIMEOUT_MULTIPLIER = 2

# Number of parallel processes for executing tests.
PARALLELISM = 4

CROSVM_ROOT = Path(__file__).parent.parent.parent.resolve()
COMMON_ROOT = CROSVM_ROOT / "common"


class ExecutableResults(object):
    """Container for results of a test executable."""

    def __init__(
        self,
        name: str,
        binary_file: Path,
        success: bool,
        test_log: str,
        previous_attempts: List["ExecutableResults"],
        profile_files: List[Path],
    ):
        self.name = name
        self.binary_file = binary_file
        self.success = success
        self.test_log = test_log
        self.previous_attempts = previous_attempts
        self.profile_files = profile_files


class Executable(NamedTuple):
    """Container for info about an executable generated by cargo build/test."""

    binary_path: Path
    crate_name: str
    cargo_target: str
    kind: str
    is_test: bool
    is_fresh: bool

    @property
    def name(self):
        return f"{self.crate_name}:{self.cargo_target}"


class Crate(NamedTuple):
    """Container for info about crate."""

    name: str
    path: Path


def get_workspace_excludes(build_triple: Triple):
    arch = build_triple.arch
    sys = build_triple.sys
    for crate, options in CRATE_OPTIONS.items():
        if TestOption.DO_NOT_BUILD in options:
            yield crate
        elif TestOption.DO_NOT_BUILD_X86_64 in options and arch == "x86_64":
            yield crate
        elif TestOption.DO_NOT_BUILD_AARCH64 in options and arch == "aarch64":
            yield crate
        elif TestOption.DO_NOT_BUILD_ARMHF in options and arch == "armv7":
            yield crate
        elif TestOption.DO_NOT_BUILD_WIN64 in options and sys == "windows":
            yield crate


def should_run_executable(executable: Executable, target: TestTarget, test_names: List[str]):
    arch = target.build_triple.arch
    options = CRATE_OPTIONS.get(executable.crate_name, [])
    if TestOption.DO_NOT_RUN in options:
        return False
    if TestOption.DO_NOT_RUN_X86_64 in options and arch == "x86_64":
        return False
    if TestOption.DO_NOT_RUN_AARCH64 in options and arch == "aarch64":
        return False
    if TestOption.DO_NOT_RUN_ARMHF in options and arch == "armv7":
        return False
    if TestOption.DO_NOT_RUN_ON_FOREIGN_KERNEL in options and not target.is_native:
        return False
    if test_names:
        for name in test_names:
            if fnmatch.fnmatch(executable.name, name):
                return True
        return False
    return True


def list_common_crates(build_triple: Triple):
    excluded_crates = list(get_workspace_excludes(build_triple))
    for path in COMMON_ROOT.glob("**/Cargo.toml"):
        # TODO(b/213147081): remove this once common/cros_async is gone.
        if not path.parent.name in excluded_crates and path.parent.name != "cros_async":
            yield Crate(name=path.parent.name, path=path.parent)


def exclude_crosvm(build_triple: Triple):
    return "crosvm" in get_workspace_excludes(build_triple)


def cargo(
    cargo_command: str,
    cwd: Path,
    flags: List[str],
    env: Dict[str, str],
) -> Iterable[Executable]:
    """
    Executes a cargo command and returns the list of test binaries generated.

    The build log will be hidden by default and only printed if the build
    fails. In VERBOSE mode the output will be streamed directly.

    Note: Exits the program if the build fails.
    """
    message_format = "json-diagnostic-rendered-ansi" if sys.stdout.isatty() else "json"
    cmd = [
        "cargo",
        cargo_command,
        f"--message-format={message_format}",
        *flags,
    ]
    if VERBOSE:
        print("$", " ".join(cmd))
    process = subprocess.Popen(
        cmd,
        cwd=cwd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        env=env,
    )

    messages: List[str] = []

    # Read messages as cargo is running.
    assert process.stdout
    for line in iter(process.stdout.readline, ""):
        # any non-json line is a message to print
        if not line.startswith("{"):
            if VERBOSE:
                print(line.rstrip())
            messages.append(line.rstrip())
            continue
        json_line = json.loads(line)

        # 'message' type lines will be printed
        if json_line.get("message"):
            message = json_line.get("message").get("rendered")
            if VERBOSE:
                print(message)
            messages.append(message)

        # Collect info about test executables produced
        elif json_line.get("executable"):
            yield Executable(
                Path(json_line.get("executable")),
                crate_name=json_line.get("package_id", "").split(" ")[0],
                cargo_target=json_line.get("target").get("name"),
                kind=json_line.get("target").get("kind")[0],
                is_test=json_line.get("profile", {}).get("test", False),
                is_fresh=json_line.get("fresh", False),
            )

    if process.wait() != 0:
        if not VERBOSE:
            for message in messages:
                print(message)
        sys.exit(-1)


def cargo_build_executables(
    flags: List[str],
    cwd: Path = Path("."),
    env: Dict[str, str] = {},
) -> Iterable[Executable]:
    """Build all test binaries for the given list of crates."""
    # Run build first, to make sure compiler errors of building non-test
    # binaries are caught.
    yield from cargo("build", cwd, flags, env)

    # Build all tests and return the collected executables
    yield from cargo("test", cwd, ["--no-run", *flags], env)


def build_common_crate(build_env: Dict[str, str], crate: Crate):
    print(f"Building tests for: common/{crate.name}")
    return list(cargo_build_executables([], env=build_env, cwd=crate.path))


def build_all_binaries(target: TestTarget, crosvm_direct: bool, instrument_coverage: bool):
    """Discover all crates and build them."""
    build_env = os.environ.copy()
    build_env.update(test_target.get_cargo_env(target))
    if instrument_coverage:
        build_env["RUSTFLAGS"] = "-C instrument-coverage"

    print("Building crosvm workspace")
    features = BUILD_FEATURES[str(target.build_triple)]
    extra_args: List[str] = []
    if crosvm_direct:
        features += ",direct"
        extra_args.append("--no-default-features")

    # TODO(:b:241251677) Enable default features on windows.
    if target.build_triple.sys == "windows":
        extra_args.append("--no-default-features")

    cargo_args = [
        "--features=" + features,
        f"--target={target.build_triple}",
        "--verbose",
        "--workspace",
        *[f"--exclude={crate}" for crate in get_workspace_excludes(target.build_triple)],
    ]
    cargo_args.extend(extra_args)

    yield from cargo_build_executables(
        cargo_args,
        cwd=CROSVM_ROOT,
        env=build_env,
    )

    with Pool(PARALLELISM) as pool:
        for executables in pool.imap(
            functools.partial(build_common_crate, build_env),
            list_common_crates(target.build_triple),
        ):
            yield from executables


def get_test_timeout(target: TestTarget, executable: Executable):
    large = TestOption.LARGE in CRATE_OPTIONS.get(executable.crate_name, [])
    timeout = LARGE_TEST_TIMEOUT_SECS if large else TEST_TIMEOUT_SECS
    if target.is_native:
        return timeout
    else:
        return timeout * EMULATION_TIMEOUT_MULTIPLIER


def execute_test(target: TestTarget, attempts: int, collect_coverage: bool, executable: Executable):
    """
    Executes a single test on the given test targed

    Note: This function is run in a multiprocessing.Pool.

    Test output is hidden unless the test fails or VERBOSE mode is enabled.
    """
    options = CRATE_OPTIONS.get(executable.crate_name, [])
    args: List[str] = []
    if TestOption.SINGLE_THREADED in options:
        args += ["--test-threads=1"]

    binary_path = executable.binary_path

    # proc-macros and their tests are executed on the host.
    if executable.kind == "proc-macro":
        target = TestTarget("host")

    previous_attempts: List[ExecutableResults] = []
    for i in range(1, attempts + 1):
        if VERBOSE:
            print(f"Running test {executable.name} on {target}... (attempt {i}/{attempts})")

        try:
            # Pipe stdout/err to be printed in the main process if needed.
            test_process = test_target.exec_file_on_target(
                target,
                binary_path,
                args=args,
                timeout=get_test_timeout(target, executable),
                generate_profile=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
            )
            profile_files: List[Path] = []
            if collect_coverage:
                profile_files = [*test_target.list_profile_files(binary_path)]
                if not profile_files:
                    print()
                    print(f"Warning: Running {binary_path} did not produce a profile file.")

            result = ExecutableResults(
                executable.name,
                binary_path,
                test_process.returncode == 0,
                test_process.stdout,
                previous_attempts,
                profile_files,
            )
        except subprocess.TimeoutExpired as e:
            # Append a note about the timeout to the stdout of the process.
            msg = f"\n\nProcess timed out after {e.timeout}s\n"
            result = ExecutableResults(
                executable.name,
                binary_path,
                False,
                e.stdout.decode("utf-8") + msg,
                previous_attempts,
                [],
            )
        if result.success:
            break
        else:
            previous_attempts.append(result)

    return result  # type: ignore


def print_test_progress(result: ExecutableResults):
    if not result.success or result.previous_attempts or VERBOSE:
        if result.success:
            msg = "is flaky" if result.previous_attempts else "passed"
        else:
            msg = "failed"
        print()
        print("--------------------------------")
        print("-", result.name, msg)
        print("--------------------------------")
        print(result.test_log)
        if result.success:
            for i, attempt in enumerate(result.previous_attempts):
                print()
                print(f"- Previous attempt {i}")
                print(attempt.test_log)
    else:
        sys.stdout.write(".")
        sys.stdout.flush()


def execute_all(
    executables: List[Executable],
    target: test_target.TestTarget,
    attempts: int,
    collect_coverage: bool,
):
    """Executes all tests in the `executables` list in parallel."""

    def is_exclusive(executable: Executable):
        return TestOption.RUN_EXCLUSIVE in CRATE_OPTIONS.get(executable.crate_name, [])

    pool_executables = [e for e in executables if not is_exclusive(e)]
    sys.stdout.write(f"Running {len(pool_executables)} test binaries in parallel on {target}")
    sys.stdout.flush()
    with Pool(PARALLELISM) as pool:
        for result in pool.imap(
            functools.partial(execute_test, target, attempts, collect_coverage), pool_executables
        ):
            print_test_progress(result)
            yield result
    print()

    exclusive_executables = [e for e in executables if is_exclusive(e)]
    sys.stdout.write(f"Running {len(exclusive_executables)} test binaries on {target}")
    sys.stdout.flush()
    for executable in exclusive_executables:
        result = execute_test(target, attempts, collect_coverage, executable)
        print_test_progress(result)
        yield result
    print()


def find_crosvm_binary(executables: List[Executable]):
    for executable in executables:
        if not executable.is_test and executable.cargo_target == "crosvm":
            return executable
    raise Exception("Cannot find crosvm executable")


def generate_lcov(results: List[ExecutableResults], lcov_file: str):
    print("Merging profiles")
    merged_file = testvm.cargo_target_dir() / "merged.profraw"
    profiles = [str(p) for r in results if r.profile_files for p in r.profile_files]
    subprocess.check_call(["rust-profdata", "merge", "-sparse", *profiles, "-o", str(merged_file)])

    print("Exporting report")
    lcov_data = subprocess.check_output(
        [
            "rust-cov",
            "export",
            "--format=lcov",
            "--ignore-filename-regex='/registry/'",
            f"--instr-profile={merged_file}",
            *(f"--object={r.binary_file}" for r in results),
        ],
        text=True,
    )
    open(lcov_file, "w").write(lcov_data)


def main():
    parser = argparse.ArgumentParser(usage=USAGE)
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        default=False,
        help="Print all test output.",
    )
    parser.add_argument(
        "--target",
        default="host",
        help="Execute tests on the selected target. See ./tools/set_test_target",
    )
    parser.add_argument(
        "--build-target",
        help=(
            "Override the cargo triple to build. Shorthands are available: (x86_64, armhf, "
            + "aarch64, mingw64, msvc64)."
        ),
    )
    parser.add_argument(
        "--emulator",
        help=(
            "Specify a command wrapper to run non-native test binaries (e.g. wine64, "
            + "qemu-aarch64-static, ...)."
        ),
    )
    parser.add_argument(
        "--build-only",
        action="store_true",
    )
    parser.add_argument(
        "--generate-lcov",
        help="Generate an lcov code coverage profile",
    )
    parser.add_argument(
        "--crosvm-direct",
        action="store_true",
    )
    parser.add_argument(
        "--repeat",
        type=int,
        default=1,
        help="Repeat each test N times to check for flakes.",
    )
    parser.add_argument(
        "--retry",
        type=int,
        default=0,
        help="Retry a test N times if it has failed.",
    )
    parser.add_argument(
        "--arch",
        help="Deprecated. Please use --build-target instead.",
    )
    parser.add_argument(
        "test_names",
        nargs="*",
        default=[],
        help=(
            "Names (crate_name:binary_name) of test binaries to run "
            + "(e.g. integration_tests:boot). Globs are supported (e.g. crosvm:*)"
        ),
    )
    args = parser.parse_args()

    global VERBOSE
    VERBOSE = args.verbose  # type: ignore
    os.environ["RUST_BACKTRACE"] = "1"

    if args.arch:
        print("WARNING!")
        print("--arch is deprecated. Please use --build-target instead.")
        print()
        build_target = Triple.from_shorthand(args.arch)

    collect_coverage = args.generate_lcov
    emulator_cmd = args.emulator.split(" ") if args.emulator else None
    build_target = Triple.from_shorthand(args.build_target) if args.build_target else None
    target = test_target.TestTarget(args.target, build_target, emulator_cmd)
    print("Test target:", target)

    # Start booting VM while we build
    if target.vm:
        testvm.build_if_needed(target.vm)
        testvm.up(target.vm)

    executables = list(build_all_binaries(target, args.crosvm_direct, collect_coverage))

    if args.build_only:
        print("Not running tests as requested.")
        sys.exit(0)

    # Upload dependencies plus the main crosvm binary for integration tests if the
    # crosvm binary is not excluded from testing.
    extra_files = (
        [find_crosvm_binary(executables).binary_path]
        if not exclude_crosvm(target.build_triple)
        else []
    )

    test_target.prepare_target(target, extra_files=extra_files)

    # Execute all test binaries
    test_executables = [
        e for e in executables if e.is_test and should_run_executable(e, target, args.test_names)
    ]

    all_results: List[ExecutableResults] = []
    for i in range(args.repeat):
        if args.repeat > 1:
            print()
            print(f"Round {i+1}/{args.repeat}:")
        all_results.extend(execute_all(test_executables, target, args.retry + 1, collect_coverage))
        random.shuffle(test_executables)

    if args.generate_lcov:
        generate_lcov(all_results, args.generate_lcov)

    flakes = [r for r in all_results if r.previous_attempts]
    if flakes:
        print()
        print(f"There are {len(flakes)} flaky tests")
        for result in flakes:
            print(f"  {result.name}")

    print()
    failed = [r for r in all_results if not r.success]
    if len(failed) == 0:
        print("All tests passed.")
        sys.exit(0)
    else:
        print(f"{len(failed)} of {len(all_results)} tests failed:")
        for result in failed:
            print(f"  {result.name}")
        sys.exit(-1)
