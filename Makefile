.PHONY: test all clean

all: test 

test:
	cargo test

clean:
	rm -rf target/
