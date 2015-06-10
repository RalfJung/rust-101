FILES=$(wildcard src/*.rs)

all: docs rawsrc
.PHONY: docs rawsrc

docs:
	@./pycco-rs $(FILES)

rawsrc:
	@mkdir -p rawsrc
	@for file in $(FILES); do echo "$$file -> rawsrc/$$file"; egrep -v "^[[:space:]]*// " "$$file" > "rawsrc/""$$file"; done
