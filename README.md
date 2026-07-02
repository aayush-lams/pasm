# pasm

A password manager with a Rust CLI client and REST API backend.

- **Client** — CLI tool with master-password login, AES-256 encryption, Bearer token auth
- **Server** — Axum REST API, PostgreSQL, per-user auth key registration

---

## Quick start

```bash
# Start PostgreSQL + server
docker compose up -d

# Install the client
cargo build --bin pasm_client
./target/debug/pasm_client --help
```

Or run without Docker:

```bash
# Ensure PostgreSQL is running and set PASM_DATABASE_URL
export PASM_DATABASE_URL="postgres://user:pass@localhost/pasm"

# Start the server
cargo run --bin pasm_server

# First-time login (creates master password + registers key)
cargo run --bin pasm_client login

# Use it
cargo run --bin pasm_client create
cargo run --bin pasm_client list
cargo run --bin pasm_client find github
```

---

## Client CLI

```
Usage: pasm_client [options] <command> [args]
       pasm_client -h | --help

Global options:
  --config <path>     Config file path (default: ~/.config/pasm/config.toml)
  --addr <host:port>  Server address  (default: http://localhost:3000)

Session management:
  login              Login with master password
  logout             Log out (clear local session)

Entry management (requires login):
  create             Create an entry (interactive prompts)
  find <name>        Find and display an entry
  list               List all entries
  delete <name>      Delete an entry
  amend              Create or overwrite an entry (interactive)

Account management (requires login):
  register           Register current auth key with server
  update-auth <key>  Replace auth key (key rotation)
  remove-auth        Remove user and all data
  list-users         List all registered users
```

---

## Server

```bash
pasm_server [options]

Options:
  --config <path>     Config file path (default: ~/.config/pasm/config.toml)
  --addr <host:port>  Bind address    (default: 0.0.0.0:3000)
```

### Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PASM_DATABASE_URL` | — | PostgreSQL connection string (required) |
| `PASM_SERVER_ADDR` | `0.0.0.0:3000` | Server bind address |
| `PASM_SERVER_URL` | `http://localhost:3000` | Client-facing server URL |
| `PASM_CONFIG` | `~/.config/pasm/config.toml` | Config file path |
| `HOME` | System home | Config & session directory base |

### Config file (`~/.config/pasm/config.toml`)

```toml
server_url = "http://localhost:3000"
server_addr = "0.0.0.0:3000"
database_url = "postgres://user:pass@localhost/pasm"
max_connections = 5
```

Env vars and CLI flags override config file values.

---

## Docker Compose

```bash
docker compose up -d
```

Starts PostgreSQL 17 and pasm server on port 3000. Connection string is
auto-configured for the Compose network.

---

## API

All endpoints except `POST /auth` require `Authorization: Bearer <api_key>`.

| Method | Route | Auth | Description |
|--------|-------|------|-------------|
| `POST` | `/auth` | None | Register a new auth key |
| `GET` | `/auth/list` | Bearer | List all registered auth keys |
| `POST` | `/auth/update` | Bearer | Replace auth key (key rotation) |
| `DELETE` | `/auth/remove` | Bearer | Remove user and all entries |
| `GET` | `/entries` | Bearer | List all entries |
| `POST` | `/entry` | Bearer | Create an entry (no overwrite) |
| `POST` | `/entry/amend` | Bearer | Create or overwrite an entry |
| `GET` | `/entry/{name}` | Bearer | Find entry by name |
| `DELETE` | `/entry/{name}` | Bearer | Delete entry by name |
| `GET` | `/health` | None | Health check |

```bash
# Register
curl -X POST -H "Authorization: Bearer <key>" http://localhost:3000/auth

# Create entry
curl -X POST -H "Authorization: Bearer $KEY" \
  -H "Content-Type: application/json" \
  -d '{"key":"github","value":"encrypted-data"}' \
  http://localhost:3000/entry
```

---

## Security notes

- **Key derivation**: Two-step SHA-256: `auth_key = SHA-256("pasm-auth" + password)`, then `api_key = SHA-256(auth_key)`. The intermediate `auth_key` is never sent over the wire.
- **Entry encryption**: AES-256 client-side before upload. Server only sees ciphertext.
- **Session file**: Stored at `~/.config/pasm/session` with `0600` permissions.

### Not implemented / future
- TLS
- Argon2id key stretching
- OS keyring integration
