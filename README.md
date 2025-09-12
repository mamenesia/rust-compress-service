# Rust Compress API

A REST API for compressing and managing data, built with Rust and Axum, following the 12-factor principles.

## 12-Factor Principles Implementation

This application follows the [12-factor methodology](https://12factor.net/) for building software-as-a-service applications:

1. **Codebase** - Single codebase tracked in Git
2. **Dependencies** - Explicitly declared and isolated (Cargo.toml)
3. **Config** - Stored in environment variables (.env file for development)
4. **Backing services** - Database configuration via DATABASE_URL
5. **Build, release, run** - Separate stages (build.sh script)
6. **Processes** - Stateless, share-nothing architecture
7. **Port binding** - Exports services via port binding
8. **Concurrency** - Scales out via process model (multiple workers)
9. **Disposability** - Fast startup and graceful shutdown
10. **Dev/prod parity** - Minimal divergence between environments
11. **Logs** - Treat logs as event streams
12. **Admin processes** - Run admin tasks as one-off processes

## Features

- RESTful API for managing compressed data
- CRUD operations (Create, Read, Update, Delete)
- JSON request/response handling
- Structured logging with tracing
- Configuration via environment variables
- Health check endpoint
- PostgreSQL database storage
- Administrative CLI for database management
- Makefile for simplified commands
- Development mode with auto-reload
- OpenAPI documentation with interactive UI

## Endpoints

- `GET /` - Welcome message
- `GET /health` - Health check
- `GET /items` - Get all items
- `POST /items` - Create a new item
- `GET /items/{id}` - Get a specific item
- `PUT /items/{id}` - Update a specific item
- `DELETE /items/{id}` - Delete a specific item
- `GET /scalar` - OpenAPI documentation (Scalar UI)

## Data Model

```json
{
  "id": "string (UUID)",
  "name": "string",
  "data": "string (base64 encoded)",
  "created_at": "string (ISO 8601)",
  "updated_at": "string (ISO 8601)"
}
```

## Configuration

The application is configured through environment variables:

- `HOST` - Server host (default: 0.0.0.0)
- `PORT` - Server port (default: 3000)
- `DATABASE_URL` - PostgreSQL database connection string
- `DEBUG` - Debug mode (default: false)
- `RUST_LOG` - Log level (default: info)

## Development Setup

1. Install Rust: https://www.rust-lang.org/tools/install

2. Install cargo-watch for development mode:
   ```bash
   cargo install cargo-watch
   ```

3. Install PostgreSQL: https://www.postgresql.org/download/

4. Create a PostgreSQL database:
   ```sql
   CREATE DATABASE rust_compress_api;
   ```

5. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rust_compress_api
   ```

6. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

7. Update the DATABASE_URL in the .env file to match your PostgreSQL setup

## Running the Application

### Development Mode (Recommended)

For development with auto-reload:
```bash
make dev
```

For development with auto-reload and database:
```bash
make dev-db
```

### With Docker (Recommended for Production)

1. Start only the database service:
   ```bash
   make db-up
   ```

2. In another terminal, run the application:
   ```bash
   make run
   ```

3. The API will be available at `http://localhost:3000`

### Using Docker Compose

1. Start all services:
   ```bash
   docker-compose up
   ```

2. The API will be available at `http://localhost:3000`

### Local Development

1. Run the application:
   ```bash
   make run
   ```

2. The API will be available at `http://localhost:3000`

## API Documentation

The API includes interactive documentation using Scalar UI:

1. Start the application
2. Navigate to `http://localhost:3000/scalar` in your browser
3. Explore and test all API endpoints

The documentation includes:
- Detailed descriptions of all endpoints
- Request/response schemas
- Example requests and responses
- Interactive testing capabilities

## Administrative Tasks

The application includes a CLI tool for administrative tasks:

Using Makefile commands:
```bash
# Count total items in database
make admin-count

# Show database statistics
make admin-stats

# Delete all items from database (requires confirmation)
make admin-clear
```

Or using cargo directly:
```bash
# Count total items in database
cargo run --bin admin -- count

# Show database statistics
cargo run --bin admin -- stats

# Delete all items from database (requires confirmation)
cargo run --bin admin -- clear --confirm
```

## Building for Production

Using Makefile:
```bash
make build
```

Or using cargo directly:
```bash
cargo build --release
```

The binaries will be located at:
- `target/release/rust_compress_api` - Main API server
- `target/release/admin` - Administrative CLI tool

## API Usage Examples

### Create an item

```bash
curl -X POST http://localhost:3000/items \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Data",
    "data": "SGVsbG8gV29ybGQ="
  }'
```

### Get all items

```bash
curl http://localhost:3000/items
```

### Get a specific item

```bash
curl http://localhost:3000/items/{id}
```

### Update an item

```bash
curl -X PUT http://localhost:3000/items/{id} \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Name"
  }'
```

### Delete an item

```bash
curl -X DELETE http://localhost:3000/items/{id}
```

## Logging

The application uses structured logging with different levels (error, warn, info, debug, trace). 
Set the `RUST_LOG` environment variable to control the log level.

## Deployment

This application can be deployed to any cloud platform that supports running Rust binaries. 
It follows the 12-factor principles making it suitable for containerized deployments (Docker, Kubernetes) 
or platform-as-a-service providers (Heroku, etc.).
