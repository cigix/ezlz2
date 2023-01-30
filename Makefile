all: ezlz unezlz

build:
	cargo build --profile release

ezlz: build
	cp target/release/ezlz .

unezlz: build
	cp target/release/unezlz .

.PHONY: all build
