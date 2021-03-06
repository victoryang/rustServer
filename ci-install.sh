#!/bin/sh

# Instarll arm tool
sudo apt-get update
sudo apt-get install -qq gcc-arm-linux-gnueabihf
rustup target add armv7-unknown-linux-gnueabihf

# registry ustc
echo "[source.crates-io]
registry = \"https://github.com/rust-lang/crates.io-index\"
replace-with = 'ustc'
		 
[source.ustc]
registry = \"git://mirrors.ustc.edu.cn/crates.io-index\"

[target.armv7-unknown-linux-gnueabihf]
linker = \"arm-linux-gnueabihf-gcc\"" > $HOME/.cargo/config

# Dependency for openssl
wget https://www.openssl.org/source/openssl-1.0.2l.tar.gz
tar xzf openssl-1.0.2l.tar.gz

cd openssl-1.0.2l

# dynamic library
export MACHINE=armv7
export ARCH=arm
export CC=arm-linux-gnueabihf-gcc 
./config shared && make && sudo make install

cd -
rm -rf openssl-1.0.2l*

# reset CC
unset CC
export CC_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc

# Dependency for libsqlitedb.so
sudo cp mcsql-sys/build/lib/libz.so.1.2.8 /usr/arm-linux-gnueabihf/lib/
sudo ln -s /usr/arm-linux-gnueabihf/lib/libz.so.1.2.8 /usr/arm-linux-gnueabihf/lib/libz.so.1