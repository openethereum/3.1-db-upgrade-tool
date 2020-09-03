#!/bin/bash
set -e # fail on any error
set -u # treat unset variables as error
# NOTE: Enables the aes-ni instructions for RustCrypto dependency.
# If you change this please remember to also update .cargo/config
export RUSTFLAGS=" -Ctarget-feature=+aes,+sse2,+ssse3 -Ctarget-feature=+crt-static  -Clink-arg=-s"

echo "_____ Build _____"
time cargo build --verbose --release

