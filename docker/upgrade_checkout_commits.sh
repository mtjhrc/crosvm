#!/bin/bash
# Copyright 2019 The Chromium OS Authors. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

set -ex
cd "${0%/*}"

remotes=(
    "https://github.com/mesonbuild/meson"
    "https://github.com/anholt/libepoxy.git"
    "https://chromium.googlesource.com/chromiumos/third_party/tpm2"
    "https://chromium.googlesource.com/chromiumos/platform2"
)

keys=(
    "MESON_COMMIT"
    "LIBEPOXY_COMMIT"
    "TPM2_COMMIT"
    "PLATFORM2_COMMIT"
)

for remote in "${remotes[@]}"; do
    remote_chunk=$(git ls-remote --exit-code "${remote}" refs/heads/master)
    commit=$(echo "${remote_chunk}" | cut -f 1 -)
    echo $commit
done
