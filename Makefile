test:
	$(MAKE) -C tests/files/generators/python
	RUSTFLAGS="-Z macro-backtrace" cargo +nightly test -- --nocapture
	$(MAKE) -C tests/files/decoders/python