.PHONY: dev build
dev:
	cd frontend && pnpm i && pnpm run dev &
	cargo run --release --bin refnet_server

build:
	cd frontend && pnpm i && pnpm run build &
	cargo build --release --bin refnet_server

run:
	cd frontend && pnpm i && pnpm run build && pnpm run preview &
	cargo run --release --bin refnet_server
