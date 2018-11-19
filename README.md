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

## Get Binary	
    rust_server

## Debug
	RUST_BACKTRACE=1
	https://stackoverflow.com/questions/38803760/how-to-get-a-release-build-with-debugging-information-when-using-cargo