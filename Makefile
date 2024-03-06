
.PHONY: setup build install installer test

setup:
	@./scripts/setup

build: setup
	@echo "Please wait while building..."
	@cargo build --release

installer: setup build
	@./scripts/build_installer

install:
	@./scripts/install

test: setup
	@./scripts/test


