.PHONY: help build run dev dev-db start-db stop-db logs-db test clean admin-count admin-stats admin-clear

# Default target
help:
	@echo "Rust Compress API - Makefile Commands"
	@echo ""
	@echo "Available commands:"
	@echo "  build          - Build the application"
	@echo "  run            - Run the main API server"
	@echo "  dev            - Run the main API server in development mode (with cargo-watch)"
	@echo "  dev-db         - Run the main API server in development mode with database"
	@echo "  db-up          - Start only the database service"
	@echo "  up             - Start all services (API + DB)"
	@echo "  stop           - Stop all services"
	@echo "  down           - Stop all services"
	@echo "  logs-db        - View database logs"
	@echo "  test           - Run tests"
	@echo "  clean          - Clean build artifacts"
	@echo "  admin-count    - Count items in database"
	@echo "  admin-stats    - Show database statistics"
	@echo "  admin-clear    - Clear all items from database"

# Build the application
build:
	cargo build --release

# Run the main API server
run:
	cargo run --bin rust_compress_api

# Run the main API server in development mode (with cargo-watch)
dev:
	cargo watch -x "run --bin rust_compress_api"

# Run the main API server in development mode with database
dev-db:
	@echo "Starting database service..."
	@docker compose up -d db
	@echo "Waiting for database to be ready..."
	@sleep 5
	@echo "Starting API server in development mode..."
	cargo watch -x "run --bin rust_compress_api"

# Start only the database service
db-up:
	docker compose up -d db

# Start all services (API + DB)
up:
	docker compose up -d

# Stop all services
stop:
	docker compose down

down:
	docker compose down

# View database logs
logs-db:
	docker compose logs -f db

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Administrative commands
admin-count:
	cargo run --bin admin -- count

admin-stats:
	cargo run --bin admin -- stats

admin-clear:
	cargo run --bin admin -- clear --confirm
