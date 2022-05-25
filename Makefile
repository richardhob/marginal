.PHONY: test all clean

all: test 

test:
	cargo test --manifest-path=sed/Cargo.toml

clean:
	rm -rf ./sed/target/
