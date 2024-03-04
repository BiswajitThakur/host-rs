prog :=host-rs

.PHONY: build install

build:
	@echo "Please wait while building..."
	cargo build --release

install:
	@cp target/release/$(prog) /usr/bin/$(prog)
	@chmod +x /usr/bin/$(prog)

installer:
	@./scripts/build_installer

setup:
	@./scripts/setup


