

PROJECTS:=sed mkdir
BIN:=$(addprefix bin/,$(PROJECTS))

get_src = $(wildcard $(1)/src/*.rs) $(wildcard $(1)/tests/*.rs)

SED_SRC:=$(call get_src,sed)
MKDIR_SRC:=$(call get_src,mkdir)

.PHONY: test all clean build_sed

all: $(BIN)

bin/sed: $(SED_SRC)
	cargo build --manifest-path=$(@F)/Cargo.toml
	cargo test --manifest-path=$(@F)/Cargo.toml
	mkdir -p $(@D)
	cp $(@F)/target/debug/$(@F) $@

bin/mkdir: $(MKDIR_SRC)
	cargo build --manifest-path=$(@F)/Cargo.toml
	cargo test --manifest-path=$(@F)/Cargo.toml
	mkdir -p $(@D)
	cp $(@F)/target/debug/$(@F) $@


clean:
	rm -rf $(BIN)
	cargo clean --manifest-path=sed/Cargo.toml
	cargo clean --manifest-path=mkdir/Cargo.toml
