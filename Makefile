web:
	cargo build --release --target wasm32-unknown-unknown
	cp -r target/wasm32-unknown-unknown/release/barbells.wasm prod

run:
	cargo run
