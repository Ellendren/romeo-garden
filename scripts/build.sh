#!/bin/bash

cargo_source="/var/lib/jenkins/.cargo/env"
source $cargo_source

echo -e "${BLUE}running rustup"
rustup update

cargo build --release