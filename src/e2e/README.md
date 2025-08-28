# End-to-End Tests

This directory contains end-to-end tests for the persons-only-votations application that connect to a running People chain node.

## Prerequisites

1. **Running People Chain Node**: Ensure you have a People chain node running and accessible
2. **Application Server**: The application server **must be running** and accessible at the configured URL
3. **Environment Configuration**: Set up the required environment variables

### Starting the Application Server

Before running e2e tests, you must start the application server:

```bash
# Requires running People chain
cargo run
```

The server should be running on `http://127.0.0.1:3000` (or your configured `E2E_SERVER_BASE_URL`).

## Running Tests

```bash
# Terminal 1: Start server (requires People chain)
cargo run

# Terminal 2: Run tests against chain
cargo test e2e

# Or using environment files
cargo test e2e --env-file .env.e2e
```

### Specific Test Categories
```bash
# Basic connectivity tests
cargo test test_people_chain_connection

# Server health check
cargo test test_server_health_check

# Poll management tests
cargo test test_basic_poll_creation_and_retrieval
cargo test test_poll_voting_without_identity_proof

# Identity proof tests
cargo test test_poll_voting_with_identity_proof --ignored
```
