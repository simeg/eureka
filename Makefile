.PHONY: check ci clippy fmt install lint publish test

BIN_NAME = eureka
CARGO = $(shell which cargo)

check:
	$(CARGO) check --release

ci: lint clippy check test

clippy:
	@$(CARGO) clippy --fix --allow-dirty

fmt:
	@$(CARGO) fmt

install:
	@cp ./target/release/$(BIN_NAME) /usr/local/bin/$(BIN_NAME)

lint:
	@$(CARGO) fmt --all -- --check && echo "Lint OK 👌"

publish:
	@$(CARGO) publish

release:
	@$(CARGO) build --release

test:
	@$(CARGO) test -- --nocapture --test-threads=1 && echo "Tests OK 👌"
