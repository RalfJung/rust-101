FILES=$(wildcard src/*.rs)
DOCFILES=$(addsuffix .html,$(addprefix docs/,$(notdir $(basename $(FILES)))))
WORKSPACEFILES=$(addprefix workspace/,$(FILES))

all: docs workspace crates
.PHONY: docs workspace crates

# Documentation
docs: $(DOCFILES)

.tmp/docs/%.rs: src/%.rs Makefile
	@mkdir -p .tmp/docs
	@echo "$< -> $@"
	@sed 's|^\(\s*//\)@|\1|;s|\s*/\*@\*/||' $< > $@

docs/%.html: .tmp/docs/%.rs
	@./pycco-rs $<

# Workspace
workspace: $(WORKSPACEFILES)

workspace/src/%.rs: src/%.rs Makefile dup-unimpl.sed
	@mkdir -p .tmp/docs
	@echo "$< -> $@"
	@sed '/^\s*\/\/@/d;s|\(\s*\)\S.*/\*@\*/|\1unimplemented!()|' $< | sed -f dup-unimpl.sed > $@

workspace/src/main.rs:
	# Don't touch this file

# Crates
crates: $(WORKSPACEFILES)
	@cargo build
	@cd solutions && cargo build
	@cd workspace && cargo build
