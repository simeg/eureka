.PHONY: build clean link install release run

build:
	cargo build

clean:
	rm -rf ./target

link:
	ln -sf ./target/debug/idea .

install:
	cp ./target/release/idea /usr/local/bin/idea

release:
	cargo build --release

run:
	RUST_BACKTRACE=1 cargo run

