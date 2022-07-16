#!/bin/bash

new_branch=$1
branching_branch=$2
if [ -z "$2" ]
  then
    branching_branch="testing"
fi

start_from_branching="git checkout $branching_branch && git pull origin $branching_branch"
eval $start_from_branching

fetch_all="git fetch"
eval $fetch_all

create_new_branch="git checkout -b $new_branch"
eval $create_new_branch
