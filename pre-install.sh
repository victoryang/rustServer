#!/bin/sh

curl https://sh.rustup.rs -sSf > rust_install.sh
chmod +x rust_install.sh && ./rust_install.sh -y
source $HOME/.cargo/env

# Install nightly
rustup install nightly
rustup default nightly

# Instarll arm tool
sudo apt-get install -qq gcc-arm-linux-gnueabihf
rustup target add armv7-unknown-linux-gnueabihf

# registry ustc
echo -e "[source.crates-io]\nregistry = \"https://github.com/rust-lang/crates.io-index\"\nreplace-with = 'ustc'\n" > $HOME/.cargo/config
echo -e "[source.ustc]\nregistry = \"git://mirrors.ustc.edu.cn/crates.io-index\"\n" >> $HOME/.cargo/config
echo -e "[target.armv7-unknown-linux-gnueabihf]\nlinker = \"arm-linux-gnueabihf-gcc\"\n" >> $HOME/.cargo/config

# Dependency for openssl
wget https://www.openssl.org/source/openssl-1.0.2l.tar.gz
tar xzf openssl-1.0.2l.tar.gz

cd openssl-1.0.2l

# dynamic library
export MACHINE=armv7
export ARCH=arm
export CC=arm-linux-gnueabihf-gcc 
./config shared && make && make install

cd -
rm -rf openssl-1.0.2l*

# reset CC
unset CC
export CC_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc

# Dependency for libsqlitedb.so
cp mcsql-sys/build/lib/libz.so.1.2.8 /usr/arm-linux-gnueabihf/lib/
ln -s /usr/arm-linux-gnueabihf/lib/libz.so.1.2.8 /usr/arm-linux-gnueabihf/lib/libz.so.1