

PROJECTS:=sed
BIN:=$(addprefix bin/,$(PROJECTS))

.PHONY: test all clean

all: $(BIN)

bin/%: %
	cargo build --manifest-path=$</Cargo.toml
	cargo test --manifest-path=$</Cargo.toml
	mkdir -p bin
	cp $</target/debug/$< $@

clean:
	rm -rf ./bin
	cargo clean --manifest-path=sed/Cargo.toml
