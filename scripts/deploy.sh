#!/bin/bash
# load user vars
branch=$1
echo $branch

user_vars="/var/lib/jenkins/.bashrc"
source $user_vars

#build target
target="release"
deploy_dir="$RMGBIN/$branch/$target"
build_bin="target/$target/rm_server"

if [[ ! -e $deploy_dir ]]; then
    mkdir -p $deploy_dir
fi

cp -r $build_bin $deploy_dir