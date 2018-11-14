CARGO = cargo
MAKE = make
MRJ = mrj-sys/mrj/
MCSQL = mcsql-sys/mcsql/
.PHONY: all

all: param-server

param-server:
	OPENSSL_LIB_DIR=/usr/local/ssl/lib OPENSSL_INCLUDE_DIR=/usr/local/ssl/include $(CARGO) build --target=armv7-unknown-linux-gnueabihf

clean:
	$(MAKE) -C $(MRJ) clean
	$(MAKE) -C $(MCSQL) clean
	rm param-server