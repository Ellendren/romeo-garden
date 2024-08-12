#!/bin/bash

BLUE='\033[0;34m'

echo -e "${BLUE}Starting build script"

#download rustc if not found
if ! [ -x "$(command -v rustc)" ]
then
    RUSRTUP="tmp-rustup.sh"
    echo -e "${BLUE}Intsalling rustc"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > $RUSRTUP
    chmod a+x $RUSRTUP
    ./$RUSRTUP -y
    rm $RUSRTUP
    source ~/.cargo/env
else
    echo -e "${BLUE}rustc is installed"
fi

echo -e "${BLUE}running rustup"
rustup update

cargo build --release