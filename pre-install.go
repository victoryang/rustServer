#!/bin/sh

curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

# registry ustc
echo -e "[source.crates-io]\nregistry = \"https://github.com/rust-lang/crates.io-index\"\nreplace-with = 'ustc'\n" > ~/.cargo/config
echo -e "[source.ustc]\nregistry = \"git://mirrors.ustc.edu.cn/crates.io-index\"\n" >> ~/.cargo/config

# Instarll arm tool
sudo apt-get install -qq gcc-arm-linux-gnueabihf
rustup target add armv7-unknown-linux-gnueabihf
echo -e "[target.armv7-unknown-linux-gnueabihf]\nlinker = \"arm-linux-gnueabihf-gcc\"\n" >> ~/.cargo/config

# Dependency for openssl
sudo apt-get install pkg-config libssl-dev