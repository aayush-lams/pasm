# pasm

A minimal password manager with a Rust CLI client and REST API backend.

- **Client** — CLI tool with master-password login, AES-256 encryption, Bearer token auth
- **Server** — Axum REST API, Sled embedded database, per-user auth key registration

---

## Stack

- **Rust** — Axum for routing, Sled as embedded database
- **AES-256** (MagicCrypt) for encrypting entry data before upload
- **SHA-256** for key derivation (auth + encryption keys from master password)
- **Bearer token** authentication via per-user registered keys
- **CLI client** — shells out to `curl` for HTTP

---

## Quick start

```bash
# Build from scratch
cargo build

# Start the server
cargo run --bin pasm_server

# In another terminal — first-time login (creates master password + registers key)
cargo run --bin pasm_client login

# Use the client
cargo run --bin pasm_client create
cargo run --bin pasm_client list
cargo run --bin pasm_client find github
cargo run --bin pasm_client logout
```

---

## Client CLI

```
Usage: pasm_client <command> [args]
       pasm_client help
       pasm_client -h | --help

Session management:
  login              Login with master password
  logout             Log out (clear session)

Entry management (requires login):
  create             Create an entry (interactive)
  find <name>        Find and display an entry
  list               List all entries
  delete <name>      Delete an entry
  amend              Amend an entry (interactive, creates if missing)

Account management (requires login):
  register           Register current auth key with server
  update-auth <key>  Replace auth key (key rotation)
  remove-auth        Remove user and all data from server
  list-users         List all registered users

Other:
  help, -h, --help   Show this help message
```

### Examples

```bash
# Start fresh — creates master password, registers key on server
cargo run --bin pasm_client login

# After login — commands use session keys automatically
cargo run --bin pasm_client list
cargo run --bin pasm_client create

# Find and amend
cargo run --bin pasm_client find github
cargo run --bin pasm_client amend

# Log out — clears session, commands will refuse until next login
cargo run --bin pasm_client logout
```

---

## Running with Docker

```bash
docker build -t pasm .
docker run --rm -p 3000:3000 pasm
```

The server does **not** require an `API_KEY` environment variable when using cli. Auth keys are registered per-user via `POST /auth` at runtime.

Configurable via environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `HOME` | System home | Used to locate the Sled database at `$HOME/.config/pasm/database` |

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

### Example

```bash
# Register a new auth key (the Bearer token IS the key being registered)
curl -X POST -H "Authorization: Bearer <your_derived_key>" http://localhost:3000/auth

# List all entries
curl -H "Authorization: Bearer $API_KEY" http://localhost:3000/entries

# Create entry (key = entry name, value = encrypted data)
curl -X POST -H "Authorization: Bearer $API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"key":"github","value":"encrypted-data"}' \
  http://localhost:3000/entry

# Find entry by name
curl -H "Authorization: Bearer $API_KEY" http://localhost:3000/entry/github

# Delete entry
curl -X DELETE -H "Authorization: Bearer $API_KEY" \
  http://localhost:3000/entry/github
```

---

## Security notes

- **Key derivation**: `auth_key = SHA-256("pasm-auth" + password)`, then `api_key = SHA-256(auth_key)`. The intermediate `auth_key` is never sent over the wire.
- **Entry encryption**: Data is AES-256 encrypted client-side before upload. The server only ever sees ciphertext.

## Not implemented/ Future enhancements
- TLS,
- key stretching (Argon2id)
- OS keyring integration.
