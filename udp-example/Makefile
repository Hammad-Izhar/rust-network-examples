all:
	cargo build --release
	cp target/release/listener .
	cp target/release/sender .

clean:
	cargo clean
	rm -f listener sender