#!/bin/bash
# load user vars
user_vars="~/.bashrc"
source $user_vars

#build target
target="release"
deploy_dir="$RMBIN/$target"
build_bin="target/$target/$RMGDIR"

cp $build_bin $deploy_dir