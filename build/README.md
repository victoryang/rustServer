
## Change repo to ustc
### modify ~/.cargo/config
	[source.crates-io]
	replace-with = 'ustc'

	[source.ustc]
	registry = "git://mirrors.ustc.edu.cn/crates.io-index"

## cargo update
### To update crates

## openssl
### apt install -y libssl-dev

## Build
[example]
https://github.com/alexcrichton/curl-rust/blob/master/curl-sys/Cargo.toml

[course]
https://kornel.ski/rust-sys-crate

## Cross
1.PKG_CONFIG_ALLOW_CROSS=1 cargo build --target=armv7-unknown-linux-gnueabihf
2.openssl should be cross-compiled first, then point the env OPENSSL_DIR to installation directory
https://github.com/sfackler/rust-openssl/issues/897
https://github.com/japaric/rust-cross#cross-compiling-with-cargo
https://github.com/Yadoms/yadoms/wiki/Cross-compile-for-raspberry-PI
https://stackoverflow.com/questions/37375712/cross-compile-rust-openssl-for-raspberry-pi-2
	
###Install cross compile tools
	apt-get install git cmake libssl-dev libgnutls-dev libopencv-gpu-dev autoconf automake libtool curl make g++ unzip

### cross compile openssl
	wget https://www.openssl.org/source/openssl-1.0.2l.tar.gz
	tar xzf openssl-1.0.2l.tar.gz
	export MACHINE=armv7 && export ARCH=arm
	cd openssl-1.0.2l
	./config shared
	make CC="arm-linux-gnueabihf-gcc" AR="arm-linux-gnueabihf-ar r" RANLIB="arm-linux-gnueabihf-ranlib"
	make install

### link to crate openssl
	OPENSSL_STATIC=1 OPENSSL_LIB_DIR=/usr/local/ssl/lib OPENSSL_INCLUDE_DIR=/usr/local/ssl/include cargo build --target=armv7-unknown-linux-gnueabihf



