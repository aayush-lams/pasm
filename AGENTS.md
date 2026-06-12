# AGENTS.md

## Project Overview
**pasm** is a password manager with a Rust CLI client and REST API backend. It uses Axum for routing, PostgreSQL for storage, AES-256 encryption for entries, and Bearer token authentication.

---

## Build/Lint/Test Commands

### Build
```bash
cargo build
```

### Run
```bash
cargo run --bin pasm_server
```

### Run Single Test
```bash
cargo test <test_name>
cargo test --test <integration_test_name>
```

### All Tests
```bash
cargo test
```

### Check (Lint/Type Check)
```bash
cargo check
```

### Clippy (Linting)
```bash
cargo clippy
cargo clippy -- -D warnings
```

### Format Check
```bash
cargo fmt --check
```

### Format
```bash
cargo fmt
```

### Full Quality Check (before commit)
```bash
cargo fmt && cargo clippy -- -D warnings && cargo test
```

---

## Code Style Guidelines

### Formatting
- Use `cargo fmt` for automatic formatting (4-space indent)
- No trailing whitespace
- Maximum line length: 100 characters (soft)

### Imports
- Group imports in order: std → external crates → local modules
- Use empty lines between import groups
- Example:
  ```rust
  use std::sync::Arc;
  
  use axum::{extract::State, routing::get, Router};
  use serde::{Deserialize, Serialize};
  
  use crate::types::state::PasmState;
  ```

### Naming Conventions
- **Structs**: `PascalCase` (e.g., `PasmState`, `Details`)
- **Enums**: `PascalCase` variants (e.g., `PasmErrors::DecryptionError`)
- **Functions**: `snake_case` (e.g., `serialize_entry`, `decrypt_string`)
- **Variables**: `snake_case` (e.g., `api_key`, `encr_key`)
- **Modules**: `snake_case` (e.g., `api`, `auth`, `types`)
- **Constants**: `SCREAMING_SNAKE_CASE` if truly constant, otherwise follow variable rules

### Types
- Use `Arc<T>` for shared ownership of non-Copy types (see `state.rs:10-11`)
- Prefer explicit error types over generic `Box<dyn Error>`
- Use `serde::{Serialize, Deserialize}` with derive macros for serialization

### Error Handling
- Define custom errors in `types/error.rs` with structured variants
- Pattern: `#[derive(Debug)] pub enum PasmErrors`
- Wrap external errors (e.g., `MagicCryptError`, `serde_json::Error`) in custom variants
- Return `Result<T, CustomError>` for fallible operations
- Use `map_err` for error conversion in `?` operator chains

### Async/Await
- Use `async` for all Axum route handlers
- Mark `pub async fn` for public API functions
- Use `#[tokio::main]` for binary entry points

### Documentation
- Add doc comments (`///`) for public functions and types
- Include `# Example` blocks in doc comments where helpful
- Document error conditions with `# Error` or `# Errors`

### Axum Patterns
- Route handlers: `pub async fn call(...) -> impl IntoResponse`
- Extract state with `State<PasmState>`
- Extract path parameters with `Path<T>`
- Extract JSON with `Json<T>`
- Return `(StatusCode, body).into_response()` for explicit status codes

### Database Patterns (PostgreSQL via sqlx)
- All DB operations go through the `Db` trait in `types/db.rs`
- `PgDb` is the PostgreSQL implementation, wrapped in `PasmState`
- Every DB call is async — always use `.await`
- Connection pool configured via config file or `PASM_DATABASE_URL` env var
- Schema migrations live in `server/sql/migrations/` and run on startup
- Queries use `sqlx::query` / `sqlx::query_scalar` with `$N` positional binds

### Module Organization
```
src/
├── bin/          # Binary entry points (pasm_client, pasm_server)
├── lib.rs        # Library root (exports modules)
├── server/       # Server/routing logic
│   ├── api/      # Route handlers (create, find, list, delete, amend, health)
│   │   └── auth/ # Auth handlers (register, update, remove)
│   ├── auth.rs   # Auth middleware
│   └── sql/      # Schema migrations + SQL reference
├── types/        # Data types and errors
│   ├── db.rs     # Db trait + PgDb implementation
│   ├── detail.rs # Entry detail structure
│   ├── entry.rs  # RequestData (key/value)
│   ├── error.rs  # PasmResult error enum
│   ├── health.rs # HealthResponse struct
│   └── state.rs  # PasmState (app state)
└── utils/        # Helper functions
    ├── config.rs # Env / config-file / CLI flag resolution
    ├── decrypt.rs
    ├── deserialize.rs
    ├── encrypt.rs
    └── serialize.rs
```

### Testing
- Unit tests can be inline with `#[cfg(test)]` modules
- Integration tests in `tests/` directory (if added)
- Use `assert!`, `assert_eq!`, `assert_ne!` for assertions

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PASM_DATABASE_URL` | — | PostgreSQL connection string (required) |
| `PASM_SERVER_ADDR` | `0.0.0.0:3000` | Server bind address |
| `PASM_SERVER_URL` | `http://localhost:3000` | Client-facing server URL |
| `PASM_CONFIG` | `~/.config/pasm/config.toml` | Config file path |
| `HOME` | System home | Config & session directory base |

---

## Common Patterns

### Creating a New API Endpoint
1. Add module to `src/server/api/mod.rs`
2. Create handler function returning `impl IntoResponse`
3. Register route in `src/server/mod.rs`

### Adding New Error Types
1. Add variant to `PasmErrors` in `types/error.rs`
2. Handle in error conversion code as needed

### Working with Database
1. Access via `state.db` in route handlers (import `Db` trait for method visibility)
2. All DB methods are async — call with `.await`
3. Serialize data before insertion (use `utils/serialize.rs`)
4. Deserialize after retrieval (use `utils/deserialize.rs`)
5. Pool configured via config file (`max_connections`) or `PASM_DATABASE_URL` env var
