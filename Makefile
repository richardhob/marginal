

PROJECTS:=sed mkdir
BIN:=$(addprefix bin/,$(PROJECTS))

SED_SRC:=$(wildcard sed/src/*.rs) $(wildcard sed/tests/*.rs)
MKDIR_SRC:=$(wildcard mkdir/src/*.rs) $(wildcard mkdir/tests/*.rs)


.PHONY: test all clean build_sed

all: $(BIN)

bin/sed: $(SED_SRC)
	cargo build --manifest-path=sed/Cargo.toml
	cargo test --manifest-path=sed/Cargo.toml
	mkdir -p bin
	cp sed/target/debug/sed bin/sed

bin/mkdir: $(MKDIR_SRC)
	cargo build --manifest-path=mkdir/Cargo.toml
	cargo test --manifest-path=mkdir/Cargo.toml
	mkdir -p bin
	cp mkdir/target/debug/mkdir bin/mkdir


clean:
	rm -rf ./bin
	cargo clean --manifest-path=sed/Cargo.toml
