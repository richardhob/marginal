

PROJECTS:=sed
BIN:=$(addprefix bin/,$(PROJECTS))

.PHONY: test all clean

all: $(BIN)

bin/%: %
	cargo test  --manifest-path=$</Cargo.toml
	mkdir -p bin
	cp $</target/debug/$< $@

clean:
	rm -rf ./bin
