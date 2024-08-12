#!/bin/bash
# load user vars
user_vars="/var/lib/jenkins/.bashrc"
source $user_vars

#build target
target="release"
deploy_dir="$RMGBIN/$target"
build_bin="target/$target/rm_server"

if [[ ! -e $deploy_dir ]]; then
    mkdir $deploy_dir
fi

cp -r $build_bin $deploy_dir