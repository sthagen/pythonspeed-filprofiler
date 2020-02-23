.SHELLFLAGS := -eu -o pipefail -c
SHELL := bash
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build: filprofiler/libpymemprofile_api.so build_ext

.PHONY: build_ext
build_ext: filprofiler/libpymemprofile_api.so
	python3.8 setup.py build_ext --inplace

filprofiler/libpymemprofile_api.so: Cargo.lock memapi/Cargo.toml memapi/src/*.rs memapi/src/*.c memapi/build.rs
	rm -f filprofiler/libymemprofile_api.so
	cargo build --release
	cp -f target/release/libpymemprofile_api.so filprofiler/

test: build
	env RUST_BACKTRACE=1 cargo test
	env RUST_BACKTRACE=1 py.test

.PHONY: clean
clean:
	rm -rf target
	rm -rf filprofiler/*.so
	python setup.py clean
