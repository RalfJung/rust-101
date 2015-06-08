all: docs
.PHONY: docs

docs:
	docco src/*.rs -l linear
