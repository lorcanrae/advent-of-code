#!/bin/bash

YEAR=$1
FILE_NAME=$2

if ! pip show git-filter-repo > /dev/null 2>&1; then
    pip install git-filter-repo
fi

git filter-repo --path-glob "aoc-$YEAR/*/$FILE_NAME" --invert-paths --force
git push --set-upstream git@github.com:lorcanrae/advent-of-code.git main --force
git remote add origin git@github.com:lorcanrae/advent-of-code.git
