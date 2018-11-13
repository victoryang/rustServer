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
wget https://www.openssl.org/source/openssl-1.0.2l.tar.gz
tar xzf openssl-1.0.2l.tar.gz

cd openssl-1.0.2l
./Configure --prefix=$HOME/raspberry/depsBuild/openssl os/compiler:arm-linux-gnueabihf
make CC="arm-linux-gnueabihf-gcc" AR="arm-linux-gnueabihf-ar r" RANLIB="arm-linux-gnueabihf-ranlib"
make install

cd -
rm -rf openssl-1.0.2l*

# Install nightly
rustup install nightly