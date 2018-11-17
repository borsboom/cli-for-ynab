.PHONY: build lint test format update_docs precommit install

#@@@ IF CHANGE, ALSO UPDATE etc/azure-pipelines/windows-steps.yml
SET_TOOLCHAIN=rustup override set 1.30.1

build:
	$(SET_TOOLCHAIN)
	cargo build

lint:
	$(SET_TOOLCHAIN)
	rustup component add clippy-preview
	cargo clippy

test:
	$(SET_TOOLCHAIN)
	cargo test

format:
	$(SET_TOOLCHAIN)
	rustup component add rustfmt-preview
	cargo fmt

update_docs:
	$(SET_TOOLCHAIN)
	cargo build
	PATH=$$(PWD)/target/debug:$(PATH) etc/scripts/update_docs.sh

precommit:
	$(MAKE) format
	$(MAKE) test
	$(MAKE) update_docs

install:
	$(SET_TOOLCHAIN)
	cargo install
