FILES=$(wildcard src/*.rs)

all: docs rawsrc
.PHONY: docs rawsrc

docs:
	@docco $(FILES) -l linear

rawsrc:
	@mkdir -p rawsrc
	@for file in $(FILES); do echo "$$file -> raw$$file"; egrep -v "^[[:space:]]*// " "$$file" > "raw""$$file"; done
