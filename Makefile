.PHONY: build clean link lint install release run

build:
	cargo build

clean:
	rm -rf ./target

link:
	ln -sf ./target/debug/eureka .

lint:
	cargo build --features "clippy"

install:
	cp ./target/release/eureka /usr/local/bin/eureka

release:
	cargo build --release

run:
	RUST_BACKTRACE=1 cargo run

