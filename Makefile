watch: dev.api.watch

build: dev.api.build

run: dev.api.run

dev.api.watch: kyward-api/Cargo.toml dev.ui.build
	cd kyward-api && \
	RUST_BACKTRACE=1 cargo watch -x run
dev.api.run: kyward-api/Cargo.toml dev.ui.build
	cd kyward-api && \
	cargo run
dev.api.build: kyward-api/Cargo.toml dev.ui.build
	cd kyward-api && \
	cargo build
dev.db.migrate:
	diesel migration run
dev.db.remigrate:
	diesel migration redo
dev.ui.run:
	cd kyward-ui && \
	trunk serve
dev.ui.build:
	cd kyward-ui && \
	trunk build
dev.setup:
	rustup default nightly
	rustup target add wasm32-unknown-unknown
	cargo install trunk wasm-bindgen-cli systemfd cargo-watch cargo-raze