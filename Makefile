.PHONY: test all build

all: test clean

test:
	cargo test

clean:
	rm -rf target/
