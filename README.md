[![Build Status](https://travis-ci.org/victoryang/rustServer.svg?branch=master)](https://travis-ci.org/victoryang/rustServer)

# rust_server

## Get Source
    git clone https://github.com/victoryang/rustServer.git

## Build Requires
	see require.md

## Install
    chmod +x pre-install.sh
    . ./pre-install.sh

## Build
    source $HOME/.cargo/env && make

## Runtime Requires
	/usr/local/ssl/lib/libssl.so.1.0.0
	/usr/local/ssl/lib/libcrypto.so.1.0.0

## Get Binary	
    rust_server

## Debug
	RUST_BACKTRACE=1
	https://stackoverflow.com/questions/38803760/how-to-get-a-release-build-with-debugging-information-when-using-cargo
	https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html

	if build with --release then
	out: target/armv7-unknown-linux-gnueabihf/release/rustServer

## with valgrind
	https://www.cnblogs.com/xuanyuanchen/p/5761315.html

	wget http://valgrind.org/downloads/valgrind-3.11.0.tar.bz2
	modify configure: armv7*) to armv7*|arm*)
	
	./configure CC=arm-linux-gnueabihf-gcc CPP=arm-linux-gnueabihf-cpp CXX=arm-linux-gnueabihf-g++  --host=arm-linux --prefix=/opt/valgrind/lib

	make && make install

	cd /opt/valgrind/lib/
	scp -a bin/ lib/

	export VALGRIND_LIB=/opt/valgrind/lib/valgrind/

	libc6-dbg arm-linux-gnueabihf\libc\lib\arm-linux-gnueabihf