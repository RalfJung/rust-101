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
	@echo "// ***Remember to enable/add this part in \`main.rs\`!***" > $@
	@echo >> $@
	@sed '/^\s*\/\/@/d;s|\(\s*\)[^\s].*/\*@\*/|\1unimplemented!()|' $< | sed -f dup-unimpl.sed >> $@

workspace/src/main.rs:
	# Don't touch this file

# Crates
crates:
	@cargo build
	@cd solutions && cargo build
