# pasm

A minimal password manager backend written in Rust. Exposes a REST API for storing and managing encrypted credentials, secured behind API key authentication.

---

## Stack

- **Rust** — Axum for routing, Sled as embedded database
- **AES-256** encryption for stored entries
- **Bearer token** authentication middleware

---

## Running with Docker

<!-- ```bash -->
<!-- docker run --rm -p 3000:3000 -e API_KEY="your_key" --name pasm aayushlam/pasm -->
<!-- ``` -->
<!-- or -->
```bash
docker run --rm -p 3000:3000 -e API_KEY="your_key" pasm
```

---

## API

All endpoints except `/auth/register` require `Authorization: Bearer <API_KEY>`.

| Method | Route | Description |
|--------|-------|-------------|
| POST | `/auth/register` | Register new user |
| GET | `/auth/list` | List all users |
| POST | `/auth/update` | Update auth key |
| DELETE | `/auth/remove` | Remove user |
| GET | `/entries` | List all entries |
| POST | `/entry` | Create entry |
| POST | `/entry/amend` | Update entry |
| GET | `/entry/<name>` | Find entry by name |
| DELETE | `/entry/<name>` | Delete entry |

### Example

```bash
# Register new user (no payload needed, uses server API_KEY)
curl -X POST -H "Authorization: Bearer $API_KEY" http://localhost:3000/auth

# List all users
curl -H "Authorization: Bearer $API_KEY" http://localhost:3000/auth/list

# Update auth key (value = new auth key)
curl -X POST -H "Authorization: Bearer $API_KEY" -H "Content-Type: application/json" \
  -d '{"key":"","value":"new-auth-key"}' \
  http://localhost:3000/auth/update

# Remove user (deletes user and all entries)
curl -X DELETE -H "Authorization: Bearer $API_KEY" http://localhost:3000/auth/remove

# List all entries
curl -H "Authorization: Bearer $API_KEY" http://localhost:3000/entries

# Create entry (key = entry name, value = encrypted data)
curl -X POST -H "Authorization: Bearer $API_KEY" -H "Content-Type: application/json" \
  -d '{"key":"github","value":"encrypted-data-here"}' \
  http://localhost:3000/entry

# Find entry by name
curl -H "Authorization: Bearer $API_KEY" http://localhost:3000/entry/github

# Delete entry
curl -X DELETE -H "Authorization: Bearer $API_KEY" http://localhost:3000/entry/github

# Amend entry (updates existing or creates new)
curl -X POST -H "Authorization: Bearer $API_KEY" -H "Content-Type: application/json" \
  -d '{"key":"github","value":"new-encrypted-data"}' \
  http://localhost:3000/entry/amend
```

---

## Running from source

```bash
API_KEY=<API_KEY> cargo run --bin pasm_server
```
and for client

```bash
cargo run --bin pasm_client <API_KEY>
```
