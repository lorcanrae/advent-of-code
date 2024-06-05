#!/bin/bash

REPOSITORY=$1
FILE_NAME=$2

if ! pip show git-filter-repo > /dev/null 2>&1; then
    pip install git-filter-repo
fi

git chechout main

git filter-repo --path $FILE_NAME --invert-paths
git push origin --force --all and git push origin --force --tags
# git push --all --set-upstream git@github.com:lorcanrae/$REPOSITORY.git main --force
