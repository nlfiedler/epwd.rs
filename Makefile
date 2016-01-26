#
# Makefile to invoke cargo with the appropriate arguments depending on the
# operating system. For Darwin, we need to flatten the namespace and suppress
# undefined symbols in order for the linker to succeed.
#
# Do not invoke make directly, but rather use rebar for everything, which
# delegates to make as appropriate.
#

.PHONY: build clean

OS=$(shell uname -o)
ifeq ($(OS),Darwin)
build:
	cargo rustc -- --codegen link-args='-flat_namespace -undefined suppress'
	mkdir -p priv
	cp target/debug/libepwd_rs.dylib priv/libepwd_rs.so
else
build:
	cargo build
	mkdir -p priv
	cp target/debug/libepwd_rs.so priv/libepwd_rs.so
endif

clean:
	cargo clean
