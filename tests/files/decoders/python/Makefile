SHELL:=/bin/bash
.DEFAULT_GOAL := test

.PHONY: proto
proto:
	protoc $(shell find ../../proto -type d | xargs -I\{} echo --proto_path=\{}) --python_out=. $(shell find ../../proto -name '*.proto')

.PHONY: test
test: proto
	pip install -r requirements.txt
	python3 test.py