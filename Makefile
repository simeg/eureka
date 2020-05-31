.PHONY: build check ci clean fmt install link lint release run test

BIN_NAME = eureka
CARGO = $(shell which cargo)

build:
	@$(CARGO) build

check:
	$(CARGO) check --release

# TODO(simeg): Disabled until I can figure out how to make it pass on all releases
#ci: lint build test
ci: lint build test
	@echo "Everything's OK 🤘"

clean:
	rm -rf ./target

clippy:
	@$(CARGO) clippy

fmt: format

format:
	@$(CARGO) fmt

install:
	@cp ./target/release/$(BIN_NAME) /usr/local/bin/$(BIN_NAME)

link:
	@ln -sf ./target/debug/$(BIN_NAME) .

lint:
	@$(CARGO) fmt --all -- --check
	@echo "Lint OK 👌"

# TODO: In CI - verify that packaged .cargo file has reasonable size
package:
	@$(CARGO) package --allow-dirty

publish:
	@$(CARGO) publish

release:
	@$(CARGO) build --release

run:
	@RUST_BACKTRACE=1 $(CARGO) run

test:
	@$(CARGO) test -- --nocapture && echo "Tests OK 👌"
