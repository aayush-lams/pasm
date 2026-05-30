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
# Build everything
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

The server does **not** require an `API_KEY` environment variable. Auth keys are registered per-user via `POST /auth` at runtime.

Configurable via environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `HOME` | System home | Used to locate the Sled database at `$HOME/.config/path/database` |

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
# Register a new auth key
curl -X POST -H "Authorization: Bearer $API_KEY" http://localhost:3000/auth

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

## Module structure

```
src/
├── bin/
│   ├── pasm_client.rs       CLI entry point
│   └── pasm_server.rs       Server entry point
├── client/
│   ├── cli/commands.rs      Command enum and argument parsing
│   ├── curl/requests.rs     Curl execution and request builders
│   ├── response/display.rs  Response formatting and pretty-print
│   ├── auth/master.rs       Login/logout, session, key derivation
│   ├── entry/ops.rs         Entry CRUD with encrypt/decrypt orchestration
│   └── input/prompts.rs     Interactive user input helpers
├── server/                  Axum routes, auth middleware
├── types/                   Data types, state, errors, database wrapper
└── utils/                   Encryption, decryption, serialize, deserialize
```

---

## Security notes

- **Master password** is never stored. A verification hash (MagicCrypt-encrypted known plaintext) is kept at `~/.config/pasm/master.hash`.
- **Session file** (`~/.config/pasm/session`, permissions 0600) stores the derived `api_key` and `encr_key` in plaintext.
- **Key derivation**: `auth_key = SHA-256("pasm-auth" + password)`, then `api_key = SHA-256(auth_key)`. The intermediate `auth_key` is never sent over the wire.
- **Entry encryption**: Data is AES-256 encrypted client-side before upload. The server only ever sees ciphertext.
- No TLS, no key stretching (Argon2id), no OS keyring integration — see [`pasm-walkthrough.md`](pasm-walkthrough.md) for the full audit and upgrade roadmap.
