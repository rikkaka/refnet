.PHONY: run
run:
	@echo "Starting frontend and backend..."
	# Start the frontend
	cd frontend && npm run dev &
	# Start the backend
	cargo run --release --bin refnet_server
