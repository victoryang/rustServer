CARGO = cargo
MAKE = make
CP = cp
MRJ = mrj-sys/mrj/
MCSQL = mcsql-sys/mcsql/
.PHONY: all

all: rust_server

rust_server:
	OPENSSL_LIB_DIR=/usr/local/ssl/lib OPENSSL_INCLUDE_DIR=/usr/local/ssl/include $(CARGO) build --target=armv7-unknown-linux-gnueabihf
	$(CP) -a target/armv7-unknown-linux-gnueabihf/debug/rust_server .

clean:
	$(MAKE) -C $(MRJ) clean
	$(MAKE) -C $(MCSQL) clean
	rm rust_server