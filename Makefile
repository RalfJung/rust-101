FILES=$(wildcard src/*.rs)
DOCFILES=$(addsuffix .html,$(addprefix docs/,$(notdir $(basename $(FILES)))))
WORKSPACEFILES=$(addprefix workspace/,$(FILES))

all: docs workspace crates
.PHONY: docs workspace crates

## Documentation
docs: $(DOCFILES)

.tmp/docs/%.rs: src/%.rs Makefile pycco-rs dup-unimpl.sed
	@mkdir -p .tmp/docs
	@echo "$< -> $@"
	@# sed-fu: remove the "@" from "//@", and remove trailing "/*@*/", replace lines ending in  "/*@@*/" by "unimplemented!()".
	@# Also coalesce multiple adjacent such lines to one.
	@sed 's|^\(\s*//\)@|\1|;s|\s*/\*@\*/$$||;s|\(\s*\)\S.*/\*@@\*/|\1unimplemented!()|' $< | sed -f dup-unimpl.sed > $@

docs/%.html: .tmp/docs/%.rs
	~/.local/pipx/venvs/pycco/bin/python pycco-rs $<

## Workspace
# The generated files are shipped only for the benefit of Windows users, who
# typically don't have the necessary tools for generating the workspace
# available.
workspace: $(WORKSPACEFILES) docs/workspace.zip

workspace/src/%.rs: src/%.rs Makefile dup-unimpl.sed
	@mkdir -p .tmp/docs workspace/src/
	@echo "$< -> $@"
	@# sed-fu: remove lines starting with "//@", and replace those ending in "/*@*/" or "/*@@*/" by "unimplemented!()".
	@# Also coalesce multiple adjacent such lines to one.
	@sed '/^\s*\/\/@/d;s|\(\s*\)\S.*/\*@@\?\*/|\1unimplemented!()|' $< | sed -f dup-unimpl.sed > $@

workspace/src/main.rs:
	# Don't touch this file

docs/workspace.zip: $(WORKSPACEFILES) workspace/Cargo.toml workspace/Cargo.lock
	@rm -f $@
	zip $@ $^

## Crates
crates: $(WORKSPACEFILES)
	@cargo build
	@cd workspace && cargo build
	@cd solutions && cargo build && cargo test
