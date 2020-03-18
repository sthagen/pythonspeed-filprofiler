.SHELLFLAGS := -eu -o pipefail -c
SHELL := bash
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build: filprofiler/_filpreload.so

target/release/libpymemprofile_api.a: Cargo.lock memapi/Cargo.toml memapi/src/*.rs
	cargo build --release

venv:
	python3 -m venv venv/
	venv/bin/pip install -e .[dev]

filprofiler/_filpreload.so: filprofiler/_filpreload.c target/release/libpymemprofile_api.a
	gcc -std=c11 $(shell python3.8-config --cflags) $(shell python3.8-config --ldflags) -flto -shared -lpython3.8 -fvisibility=hidden -I$(shell python -c "import sysconfig; print(sysconfig.get_paths()['include'])") -o $@ $^

test: build
	cythonize -3 -i python-benchmarks/pymalloc.pyx
	env RUST_BACKTRACE=1 cargo test
	env RUST_BACKTRACE=1 py.test

.PHONY: docker-image
docker-image:
	docker build -t manylinux-rust -f wheels/Dockerfile.build .

.PHONY: wheel
wheel:
	docker run -u $(shell id -u):$(shell id -g) -v $(PWD):/src manylinux-rust /src/wheels/build-wheels.sh

.PHONY: clean
clean:
	rm -rf target
	rm -rf filprofiler/*.so
	python setup.py clean
