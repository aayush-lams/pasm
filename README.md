# pasm

**pasm** is a minimal password/credential manager backend written in Rust.
It exposes a REST API that allows authenticated users to create, retrieve, modify, and delete credential entries.

The goal of this project is to learn and create a secure locally hosted backend for credential management.

---

## Features

* REST API for credential storage
* API key authentication
* Encrypted entry storage
* CRUD operations for credential entries
* Environment-based secret management
* Simple architecture for experimentation and learning

---

## Architecture

The server exposes a small REST API that manages credential entries.

```
Client (curl / CLI)
      │
      ▼
Rust API Server
      │
Authentication Middleware
      │
Entry Handlers
      │
Encrypted Storage
```

Routes are implemented using a router layer and protected by authentication middleware.

---

## Running the Server

First set required environment variables:

```
export API_KEY=my_hash123
export ENCRYPTION_KEY=my_encryption_key
```

Then start the server:

```
cargo run --bin pasm_server
```

The API will run on:

```
http://127.0.0.1:3000
```

---

## Authentication

All protected endpoints require an API key in the request header:

```
Authorization: Bearer <API_KEY>
```

Example:

```
Authorization: Bearer my_hash123
```

---

## API Usage

### List Entries

```
curl http://127.0.0.1:3000/entries \
-H "Authorization: Bearer my_hash123"
```

---

### Create Entry

```
curl -X POST http://127.0.0.1:3000/entry \
-H "Authorization: Bearer my_hash123" \
-d '{"name":"github","username":"user","password":"secret"}'
```

---

### Find Entry

```
curl http://127.0.0.1:3000/entry/github \
-H "Authorization: Bearer my_hash123"
```

---

### Update Entry

```
curl -X POST http://127.0.0.1:3000/entry/amend \
-H "Authorization: Bearer my_hash123" \
-d '{ ... }'
```

---

### Delete Entry

```
curl -X DELETE http://127.0.0.1:3000/entry/github \
-H "Authorization: Bearer my_hash123"
```

---

## Security Notes

* Entries are encrypted using a server-side encryption key.
* API access requires a bearer token defined by `API_KEY`.

This project is intended for experimentation and learning secure backend practices.

---

## Future Improvements

Possible improvements include:

* JWT-based authentication
* rate limiting
* audit logging
* CLI client
* containerized deployment
