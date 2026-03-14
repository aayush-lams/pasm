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
docker run -d \
  -p 3000:3000 \
  -e API_KEY=your_key \
  -e ENCRYPTION_KEY=your_encryption_key \
  --name pasm \
  aayushlam/pasm
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
# Create
curl -X POST http://localhost:3000/entry \
  -H "Authorization: Bearer your_key" \
  -H "Content-Type: application/json" \
  -d '{"name":"github","site":"github.com","uname":"user","pword":"secret","note":""}'

# List
curl http://localhost:3000/entries \
  -H "Authorization: Bearer your_key"
```

---

## Running from source

```bash
export API_KEY=your_key
export ENCRYPTION_KEY=your_encryption_key
cargo run --bin pasm_server
```
