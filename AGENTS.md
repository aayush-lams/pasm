# AGENTS.md

## Project Overview
**pasm** is a minimal password manager backend written in Rust. It uses Axum for routing, Sled as an embedded database, AES-256 encryption for stored entries, and Bearer token authentication.

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
- Use `sled::IVec` for database key/value storage
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

### Database Patterns (Sled)
- Open database: `sled::open(path).expect("message")`
- Check existence: `db.contains_key(&name)`
- Insert: `db.insert(name, value.as_bytes())`
- Get: `db.get(key)` returns `Result<Option<IVec>>`
- Remove: `db.remove(key)` returns `Result<Option<IVec>>`
- Scan prefix: `db.scan_prefix("prefix:")` for iteration
- Key format convention: `{type}:{identifier}` (e.g., `entry:github`)

### Module Organization
```
src/
├── bin/          # Binary entry points
├── lib.rs        # Library root (exports modules)
├── server/       # Server/routing logic
│   ├── api/      # Route handlers (create, find, list, delete, amend)
│   └── auth.rs   # Authentication middleware
├── types/        # Data types and errors
│   ├── db.rs     # Database configuration/trees (TODO)
│   ├── detail.rs # Entry data structure
│   ├── error.rs  # Error enum
│   └── state.rs  # Application state
└── utils/        # Helper functions
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
Required for running:
- `API_KEY`: Bearer token for API authentication
- `ENCRYPTION_KEY`: AES-256 key for encrypting entries
- `HOME`: Used to locate config directory for database

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
1. Access via `state.db` in route handlers
2. Serialize data before insertion (use `utils/serialize.rs`)
3. Deserialize after retrieval (use `utils/deserialize.rs`)
