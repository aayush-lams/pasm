# pasm

A minimal password manager backend written in Rust. Exposes a REST API for storing and managing encrypted credentials, secured behind API key authentication.

---

## Stack

- **Rust** — Axum for routing, Sled as embedded database
- **AES-256** encryption for stored entries
- **Bearer token** authentication middleware

---

## Running with Docker

```bash
docker run --rm -p 3000:3000 -e API_KEY="your_key" --name pasm aayushlam/pasm
```
or
```bash
docker run --rm -p 3000:3000 -e API_KEY="your_key" pasm
```

---

## API

All endpoints require `Authorization: Bearer <API_KEY>`.

| Method | Route | Description |
|--------|-------|-------------|
| GET | `/entries` | List all entries |
| GET | `/entry/<name>` | Find entry by name |
| POST | `/entry` | Create entry |
| POST | `/entry/amend` | Update entry |
| DELETE | `/entry/<name>` | Delete entry |

### Example

```bash
# List all entries
curl -H "Authorization: Bearer $API_KEY" http://localhost:3000/entries

# Create entry
curl -X POST -H "Authorization: Bearer $API_KEY" -H "Content-Type: application/json" \
  -d '{"entry_name":"github","entry":"encrypted-data-here"}' \
  http://localhost:3000/entry

# Find entry by name
curl -H "Authorization: Bearer $API_KEY" http://localhost:3000/entry/github

# Delete entry
curl -X DELETE -H "Authorization: Bearer $API_KEY" http://localhost:3000/entry/github

# Amend entry
curl -X POST -H "Authorization: Bearer $API_KEY" -H "Content-Type: application/json" \
  -d '{"entry_name":"github","entry":"new-encrypted-data"}' \
  http://localhost:3000/entry/amend
```

---

## Running from source

```bash
export API_KEY=your_key
export ENCRYPTION_KEY=your_encryption_key
cargo run --bin pasm_server
```
