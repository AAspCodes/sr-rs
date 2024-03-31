all: run

run:
	cargo r . 2> err.log

fmt:
	cargo fmt

build:
	cargo build

setup:
	./scripts/setup.sh
