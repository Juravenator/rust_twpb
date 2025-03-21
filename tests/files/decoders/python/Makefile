SHELL:=/bin/bash
.DEFAULT_GOAL := test

.PHONY: test
test:
	python3 -m venv .venv
	.venv/bin/pip install -r requirements.txt
	.venv/bin/python3 test.py
