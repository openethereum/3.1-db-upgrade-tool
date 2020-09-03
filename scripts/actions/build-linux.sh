#!/bin/bash

set -e # fail on any error
set -u # treat unset variables as error
#strip ON
export RUSTFLAGS=" -Clink-arg=-s -Ctarget-feature=+aes,+sse2,+ssse3"

echo "_____ Build OpenEthereum and tools _____"

time cargo build --verbose --color=always --release
