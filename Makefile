CARGO = cargo

.PHONY: all

all: param-server

param-server:
	OPENSSL_STATIC=1 OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu/ OPENSSL_INCLUDE_DIR=/usr/include/x86_64-linux-gnu/ $(CARGO) build --target=armv7-unknown-linux-gnueabihf

clean:
	rm param-server