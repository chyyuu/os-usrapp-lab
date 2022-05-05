#!/bin/sh

git remote add -f $1 $2
git merge -s ours --no-commit $1/master
git read-tree --prefix=$1/ -u $1/master
git commit -m "merging $1 into subdirectory"
