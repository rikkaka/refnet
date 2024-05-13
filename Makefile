.PHONY: run
run:
	@echo "Starting frontend and backend..."
	# Start the frontend
	cd frontend && pnpm i && pnpm run dev &
	# Start the backend
	cargo run --release --bin refnet_server
