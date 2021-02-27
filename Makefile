.PHONY: build clean check ci clippy fmt install link lint publish test

BIN_NAME = eureka
CARGO = $(shell which cargo)

build:
	$(CARGO) build

clean:
	rm -rf ~/.eureka

check:
	$(CARGO) check --release

ci: lint check test

clippy:
	@$(CARGO) clippy

fmt:
	@$(CARGO) fmt

install:
	@cp ./target/release/$(BIN_NAME) /usr/local/bin/$(BIN_NAME)

link:
	@ln -sf ./target/debug/$(BIN_NAME) .

lint:
	@$(CARGO) fmt --all -- --check
	@echo "Lint OK 👌"

publish:
	@$(CARGO) publish

release:
	@$(CARGO) build --release

test:
	@$(CARGO) test -- --nocapture --test-threads=1 && echo "Tests OK 👌"
