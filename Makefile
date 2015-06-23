FILES=$(wildcard src/*.rs)
DOCFILES=$(addsuffix .html,$(addprefix docs/,$(notdir $(basename $(FILES)))))

all: docs crates
.PHONY: docs rawsrc crates

docs: $(DOCFILES)

docs/%.html: src/%.rs
	@./pycco-rs $^

rawsrc:
	@mkdir -p rawsrc
	@for file in $(FILES); do echo "$$file -> rawsrc/$$file"; egrep -v "^[[:space:]]*// " "$$file" > "rawsrc/""$$file"; done

crates:
	@cargo build
	@cd solutions && cargo build
