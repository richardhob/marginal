

PROJECTS:=sed
BIN:=$(addprefix bin/,$(PROJECTS))

.PHONY: test all clean build_sed

all: $(BIN)

bin/sed: sed/src/main.rs sed/src/test_utils.rs sed/tests/cli.rs sed/src/utils.rs
	cargo build --manifest-path=sed/Cargo.toml
	cargo test --manifest-path=sed/Cargo.toml
	mkdir -p bin
	cp sed/target/debug/sed bin/sed

bin/%: %
	cargo build --manifest-path=$</Cargo.toml
	cargo test --manifest-path=$</Cargo.toml
	mkdir -p bin
	cp $</target/debug/$< $@


clean:
	rm -rf ./bin
	cargo clean --manifest-path=sed/Cargo.toml
