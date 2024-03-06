
.PHONY: setup build install installer

setup:
	@./scripts/setup

build: setup
	@echo "Please wait while building..."
	@cargo build --release

installer: setup build
	@./scripts/build_installer

install:
	@./scripts/install

