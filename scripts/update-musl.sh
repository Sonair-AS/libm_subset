#!/bin/sh
# Download musl sources for `musl-math-sys`

set -eux

url=https://git.musl-libc.org/git/musl
ref=5122f9f3c99fee366167c5de98b31546312921ab
dst=../crates/musl-math-sys/musl

if ! [ -d "$dst" ]; then
    git clone "$url" "$dst" --single-branch --depth=1000
fi

git -C "$dst" fetch "$url" --depth=1
git -C "$dst" checkout "$ref"