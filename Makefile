
.PHONY: setup build install installer test

setup:
	@./scripts/setup

build:
	@./scripts/build

installer:
	@./scripts/build_installer

install:
	@./scripts/install

test:
	@./scripts/test
