# Migrate the openethereum db from 2.5.13, 2.7.2, 3.0.1 to 3.1.0

## Warnings

* ***THIS IS BETA SOFTWARE***
* ***USE AT YOUR RISK***
* ***MIGRATIONS ARE ONE-WAY ONLY***
* ***Back up your data to prevent data loss***

## Requirements

Cargo (rust package manager), Clang and LLVM (See https://github.com/rust-rocksdb/rust-rocksdb#requirements).

For Debian/Ubuntu, you should be able to run `sudo apt-get install cargo clang llvm` to successfully obtain these.

## Executing a Database Migration

Once you've successfully installed the dependencies above, simply download this repo, `cd` into it and run `cargo run "$PARITY_PATH/chains/ethereum/db/906a34e69aec8c0d/overlayrecent`, where `$PARITY_PATH` is the path to your parity files. In most cases, this will be `~/.local/share/io.parity.ethereum`. Following is a bash block for Ubuntu that should successfully update your database, assuming default installs and assuming you've installed the depenencies above:

```bash
PARITY_PATH=~/.local/share/io.parity.ethereum
cd /tmp/
git clone https://github.com/openethereum/3.1-db-upgrade-tool.git
cd 3.1-db-upgrade-tool
cargo run "$PARITY_PATH/chains/ethereum/db/906a34e69aec8c0d/overlayrecent"
# Archive node run with this
cargo run "$PARITY_PATH/chains/ethereum/db/906a34e69aec8c0d/archive"
cd ~
rm -Rf /tmp/3.1-db-upgrade-tool
```

Note that if your parity path is somewhere where you need super-user privileges to write to it, you'll need to add `sudo` before the `cargo run` line.
