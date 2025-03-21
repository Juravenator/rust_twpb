SHELL:=/bin/bash
.DEFAULT_GOAL := generate

## This protoc version is matched with the version in requirements.txt.
bin/protoc:
	test -f protoc-30.1-linux-x86_64.zip ||  curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v30.1/protoc-30.1-linux-x86_64.zip
	unzip protoc-30.1-linux-x86_64.zip 'bin*'

.PHONY: proto
proto: bin/protoc
	mkdir -p ../../generated/python
	bin/protoc $(shell find ../../proto -type d | xargs -I\{} echo --proto_path=\{}) --python_out=../../generated/python $(shell find ../../proto -name '*.proto')

.PHONY: generate
generate: proto
	python3 -m venv .venv
	.venv/bin/pip install -r requirements.txt
	.venv/bin/python3 generate.py