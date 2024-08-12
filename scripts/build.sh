#!/bin/bash

cargo_source="/var/lib/jenkins/.cargo/env"
source $cargo_source

echo -e "${BLUE}running rustup"
rustup update

# make the config.toml fole for the build
python3 scripts/conifure-toml.py $MYSQLUSER $MYSQLPASS

cargo build --release