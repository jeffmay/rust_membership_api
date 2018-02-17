#!/bin/bash

# switch to nightly
rustup default nightly

# install musl target
rustup target add x86_64-unknown-linux-musl

# build it
cargo build --release
