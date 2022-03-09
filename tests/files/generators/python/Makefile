SHELL:=/bin/bash
.DEFAULT_GOAL := generate

.PHONY: proto
proto:
	protoc --proto_path=../../proto --python_out=. ../../proto/*.proto

.PHONY: generate
generate: proto
	pip install -r requirements.txt
	python3 generate.py