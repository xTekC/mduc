#!/bin/sh

##############################
#                            #
#  Copyright (c) xTekC.      #
#  Licensed under MPL-2.0.   #
#  See LICENSE for details.  # 
#                            #
##############################

set -e

if [ -z "$1" ]; then
    echo "Please provide a tag."
    echo "Usage: ./release.sh v[X.Y.Z]"
    exit
fi

GREEN='\033[0;32m'
NC='\033[0m' # No Color

printf "Updated release to ${GREEN}$1${NC}"

# update version
sed -E -i "/^\[package\]$/,/^\[/ s/^(version = \").*\"$/\1${1#v}\"/" Cargo.toml
cargo build --profile rel-opt

# update changelog
git cliff --tag "$1" --config cliff.toml >CHANGELOG.md
git add -A
git commit -m "chore(release): prepare for $1"
git show

# generate a changelog for the tag message
changelog=$(git cliff --tag "$1" --config cliff.toml --unreleased --strip all | sed -e '/^#/d' -e '/^$/d')

# create signed tag
# git tag -s "$1" -m "$changelog"

# create unsigned tag
git tag -a "$1" -m "$changelog"

# verify signed tag
# git tag -v "$1"

echo "Done!"
echo "Push the commit (git push), wait for CI, then push the tag (git push origin v<tag_num>)."
