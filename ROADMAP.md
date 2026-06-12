# Roadmap: pasm → Full-Fledged Credential Manager

This document outlines the work needed to evolve pasm from a minimal
password manager into a production-grade credential management system.

---

## Phase 0 — Foundation & Configuration

- [ ] **Server config file initialization**
      `pasm_server --init-config` writes a default `config.toml` with
      `database_url`, `server_addr`, `max_connections` so users don't
      need to create it manually (mirrors client's auto-creation on login).

- [ ] **Metrics endpoint**
      `GET /metrics` exposing Prometheus-formatted metrics:
      request count, latency histograms, error rate, active connections,
      DB pool status, goroutine (task) count.

- [ ] **Non-localhost warning**
      Client prints a warning when connecting to anything other than
      `127.0.0.1`/`localhost` without `--insecure` or TLS.

---

## Phase 1 — Database (PostgreSQL)

- [ ] **Pagination on list endpoints**
      `GET /entries?page=2&per_page=50` returning `Link` headers
      and `{ data: [...], total, page, per_page, total_pages }`.

- [ ] **Filtering support**
      `GET /entries?q=github&category=work&sort=updated_at&order=desc`.

---

## Phase 2 — Cryptography: Argon2id + Salt

### Current problems

- Master password verification uses `MagicCrypt` (AES encrypt/decrypt of
  a known plaintext) — not a proper KDF.
- Key derivation uses single-pass `SHA-256` — trivially fast to brute-force.
- The salt is a fixed context string (`"pasm-auth"`, `"pasm-encr"`).
- The `encr_key` (AES-256 key) is derived from the password — password
  rotation requires re-encrypting all entries.

### What needs to change

- [ ] **Replace MagicCrypt password verification with Argon2id**
      Store `Argon2id(password, salt) -> hash` alongside the salt.
      The salt is a random 16-byte value generated per user at registration.

- [ ] **Replace SHA-256 key derivation with Argon2id**
      Derive a single master key from the password using Argon2id:
      ```
      master_key = Argon2id(password, salt, mem=64MB, time=3, threads=4)
      ```

- [ ] **Separate auth from encryption using HKDF**
      From `master_key`, derive two sub-keys:
      ```
      auth_key    = HMAC-SHA256(master_key, "pasm-auth")
      encr_key    = HMAC-SHA256(master_key, "pasm-encr")
      ```
      The server **never** sees `encr_key` or `master_key`.
      Only `auth_key` is sent as the Bearer token.

- [ ] **Per-entry encryption IV**
      Each entry uses a random 12-byte IV for AES-256-GCM.
      Store the IV alongside the ciphertext in `entries.encryption_iv`.

- [ ] **Encryption metadata indexing**
      Track `key_version` per entry so re-encryption can be batched
      and verified. Expose `GET /admin/encryption-status` showing
      which entries use which key version.

- [ ] **Server-side auth key hashing**
      Store `SHA-256(auth_key)` in `users.auth_key_hash`, not the raw key.
      Hash on arrival before lookup.

- [ ] **Constant-time comparison**
      Use `subtle::ConstantTimeEq` for all auth token comparisons to
      prevent timing side-channels.

- [ ] **Security tests**
      - Brute-force resistance: verify Argon2id parameters force
        minimum 100ms per derivation.
      - Timing attack: assert token comparison is constant-time.
      - Encryption: known-plaintext test with GCM authentication tag
        verification.
      - Key separation: verify auth_key and encr_key are independent
        (changing one does not affect the other).

---

## Phase 3 — Rate Limiting & Throttling

- [ ] **Per-endpoint rate limits (token bucket per IP)**

      | Endpoint      | Limit           | Why                         |
      |---------------|-----------------|-----------------------------|
      | `POST /auth`  | 5/min per IP    | Registration spam           |
      | `POST /auth`  | 30/hr per IP    | Brute-force registration    |
      | All others    | 60/min per user | General rate limit          |

- [ ] **Per-user rate limiting**
      Keyed by `user_id` from the auth middleware.
      Separate limits for read vs write operations.

- [ ] **Request throttling (leaky bucket)**
      Smooth burst traffic by queuing excess requests with a short
      delay instead of dropping them outright. Configurable queue depth.

- [ ] **Redis-backed rate limiter**
      Move from in-memory `HashMap` to Redis `INCR`+`EXPIRE` so limits
      survive restarts and work across multiple server instances.

- [ ] **Axum middleware layer**
      `RateLimitLayer` wrapping both public and protected routes.

- [ ] **`X-RateLimit-*` response headers**
      `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`.

- [ ] **Graceful degradation**
      If Redis is unreachable, fall back to a local in-memory limiter
      with reduced capacity (fail-degraded, not fail-open).

---

## Phase 4 — Redis Caching Layer

- [ ] **Session cache**
      Cache validated session tokens with their TTL so repeated auth
      checks don't hit PostgreSQL on every request.

- [ ] **Entry cache**
      Cache recently accessed entries (encrypted blobs, so no plaintext
      exposure). TTL of 5 min, invalidated on write.

- [ ] **Rate limiter storage**
      Use Redis atomic counters for per-IP and per-user rate limits.

- [ ] **Idempotency keys**
      Store idempotency keys in Redis with TTL so replaying a request
      with the same `Idempotency-Key` header returns the original
      response without executing the mutation again.

- [ ] **Cache invalidation strategy**
      Write-through: on entry update/delete, evict the cached entry
      and the user's entry list cache.

- [ ] **Connection management**
      Use `redis-rs` with `bb8` connection pool. Configurable pool size
      and timeout. Health-check pings on idle connections.

---

## Phase 5 — Transport Security

- [ ] **TLS support**
      ```bash
      pasm_server --addr 0.0.0.0:443 --tls-cert cert.pem --tls-key key.pem
      ```
      Use `axum::serve` with `rustls` TLS acceptor.

- [ ] **`--insecure` flag on client**
      Skips TLS verification for development. Prints a prominent
      warning on every use.

- [ ] **Automatic HTTP→HTTPS redirect**
      Optional companion listener on port 80.

---

## Phase 6 — Native HTTP Client (Stop Shelling Out to curl)

- [ ] **Replace `std::process::Command` curl calls** with `reqwest`
      Benefits: TLS verification, timeouts, connection pooling, proper
      error messages, no `curl` binary dependency.

- [ ] **Client-side request timeout**
      Default 10 s, configurable via `--timeout` or env var.

- [ ] **Retry with exponential backoff**
      For transient failures (connection refused, 5xx). Jitter included.

---

## Phase 7 — Session & Token Management

- [ ] **JWT-based stateless sessions**
      Issue signed JWTs (with `jsonwebtoken` crate) instead of DB-stored
      session rows. Payload: `{ user_id, token_id, iat, exp }`.
      Verifying a JWT requires no DB round-trip — pure stateless.

- [ ] **JWT refresh flow**
      - Access token: 15 min TTL.
      - Refresh token: 7 day TTL, stored hash in PostgreSQL.
      - `POST /auth/refresh` exchanges a valid refresh token for a new
        access token.

- [ ] **Server-side session invalidation**
      `DELETE /session/{token_id}` adds the `token_id` to a Redis
      denylist (checked during JWT verification).

- [ ] **Account lockout**
      Track consecutive failed auth attempts in Redis with TTL.
      Lock for 15 min after 10 failures.

- [ ] **Refresh token rotation**
      Each refresh invalidates the old refresh token and issues a new
      one, so a stolen refresh token can only be used once.

---

## Phase 8 — Background Jobs

- [ ] **Background job framework**
      Use `tokio` spawned tasks with a simple in-process job registry
      and configurable interval. For multi-server, use Redis as a
      distributed lock (e.g., `redlock`) so only one instance runs
      each job.

- [ ] **Audit log flush worker**
      Mutations write to an in-memory channel (e.g., `tokio::sync::mpsc`).
      Worker batch-inserts into `audit_log` every 5 seconds or 100
      events, whichever comes first. Keeps write-path latency low.

- [ ] **Expired session cleanup**
      Deletes expired refresh tokens from PostgreSQL and stale entries
      from the Redis denylist. Runs every 10 minutes.

- [ ] **Key rotation scheduler**
      On `POST /admin/rotate-keys`, the server generates a new
      `encr_key`. A background job re-encrypts every entry that has
      `key_version < current_version`. Progress exposed via
      `GET /admin/rotation-status`.

- [ ] **Encryption metadata reindex**
      Periodic scan to verify all entries have valid IVs and
      authentication tags. Reports mismatches to audit log.

---

## Phase 9 — Stateless API Design & Horizontal Scaling

- [ ] **Stateless API**
      - No server-side session state: JWT contains all auth info.
      - Rate limiter state in Redis (externalized).
      - Job coordination via Redis distributed locks.
      - Health/metrics endpoints return instance-local data.
      - Any instance can serve any request — no sticky sessions.

- [ ] **Horizontal scaling readiness checklist**

      | Requirement              | How pasm meets it                       |
      |--------------------------|-----------------------------------------|
      | Stateless auth           | JWT with Redis denylist                 |
      | Shared rate limit state  | Redis-backed token buckets              |
      | Shared cache             | Redis entry/session cache               |
      | Job coordination         | Redis distributed locks                 |
      | Database connection pool | PgBouncer or `deadpool-postgres`        |
      | Health checks            | `GET /health` for load balancer probes  |
      | Graceful shutdown        | SIGTERM drains connections              |
      | Configuration            | Env vars, not files                     |

- [ ] **Idempotency**
      `POST` and `DELETE` endpoints accept `Idempotency-Key` header.
      Redis stores `(key_hash, response_code, response_body)` for 24 h.
      Replay with the same key returns the original response.

- [ ] **Load balancer compatibility**
      Ensure WebSockets not required (not applicable here).
      Log format supports request IDs (`x-request-id`) for tracing
      across instances.

---

## Phase 10 — Observability

- [ ] **Structured logging**
      Replace all `println!` / `eprintln!` with `tracing` crate.
      JSON output in production, human-readable in development.
      Every log line includes: `request_id`, `user_id`, `latency_ms`,
      `endpoint`, `status_code`.

- [ ] **Metrics endpoint** (`GET /metrics` in Prometheus format)
      - Request count by endpoint + status code
      - Latency histogram (p50, p95, p99) by endpoint
      - Error rate by endpoint
      - Active DB connections (pool size, acquired, idle)
      - Redis pool health
      - Background job duration and error count
      - Rate limiter hits / misses / drops

- [ ] **Latency tracking middleware**
      Axum middleware records `Duration` for every request, attaches
      it to the audit log entry and the tracing span.

- [ ] **Error tracking**
      Catch-all error handler that:
      1. Logs the full error with backtrace
      2. Sanitizes the user-facing response (no internals leaked)
      3. Increments the error counter metric

- [ ] **Audit log with latency**
      Each audit log row includes `ip_address` and `latency_ms` for
      forensic analysis.

---

## Phase 11 — Operational Hardening

- [ ] **CORS middleware**
      Restrict to specific origins when the client is a web app.

- [ ] **Graceful shutdown**
      Handle SIGTERM/SIGINT: drain HTTP connections, flush pending
      audit log writes, close DB pool, close Redis pool, flush logs.

- [ ] **Full-DB backup**
      `pg_dump`-based script. Encrypt the dump with the master key
      before moving off-host.

- [ ] **Response sanitization**
      Ensure no error messages leak stack traces, internal paths, or
      DB details.

- [ ] **Rate limit on auth recovery**
      `POST /auth/refresh` and `POST /auth/recover` have stricter
      per-IP limits (3/min).

---

## Phase 12 — Schema Evolution & Features

- [ ] **Entry categories / tags**
      Allow organizing entries into groups (e.g., "Work", "Personal").
      Add `tags TEXT[]` column to `entries`. `GET /entries?tag=work`.

- [ ] **Custom fields**
      Beyond the current (name, site, username, password, note).
      Store as `custom_fields JSONB` alongside the fixed fields.

- [ ] **Entry history / versioning**
      `entry_versions` table storing previous encrypted values each
      time an entry is updated. `GET /entry/{name}/history` to
      list versions.

- [ ] **Secure notes**
      Entries without a password field — just freeform text.

- [ ] **URL field with auto-launch**
      Store a URL per entry; client can open it directly.

- [ ] **Password generator**
      Client-side endpoint that generates a strong random password
      with configurable length and character sets.

- [ ] **Import / export**
      CSV, JSON, or Bitwarden-compatible format for bulk migration.

---

## Phase 13 — Multi-Factor Authentication

- [ ] **TOTP support**
      During login, require a 6-digit TOTP code from an authenticator app.
      Store the TOTP secret encrypted with the user's `encr_key`.

- [ ] **Recovery codes**
      Generate 8 one-time recovery codes on MFA setup; store
      SHA-256 hashes server-side.

- [ ] **WebAuthn / FIDO2** (stretch goal)
      Hardware key support for passwordless authentication.

---

## Phase 14 — Security Testing

- [ ] **Integration security tests**
      - Brute-force: verify rate limiter blocks after N attempts.
      - Token guessing: random tokens return 401, not 200.
      - Idempotency: replay with same key returns same response.
      - Entry isolation: user A cannot access user B's entries.
      - TLS: connection without TLS to production port is refused.

- [ ] **Penetration test scenarios**
      - SQL injection (should be impossible with parameterized queries).
      - JWT alg confusion (enforce `HS256` only).
      - Cache poisoning (entries cached as encrypted blobs — verify
        no plaintext leaks into Redis).
      - Session fixation (JWT `iat` check prevents replay of tokens
        issued before password change).

- [ ] **Dependency audit**
      `cargo audit` in CI. `cargo deny` for license compliance.
      Fail build on crates with known vulnerabilities.

- [ ] **Fuzzing**
      Fuzz API input parsing (entry names, JSON payloads, auth
      headers) with `cargo-fuzz`.

---

## Summary by Dependency

```
Phase 0:  Config              ← no dependencies
Phase 1:  PostgreSQL          ← no dependencies
Phase 2:  Argon2id            ← no dependencies (can parallel Phase 1)
Phase 3:  Rate limiting       ← depends on Phase 0, Phase 4 (Redis)
Phase 4:  Redis cache         ← depends on Phase 0
Phase 5:  TLS                 ← depends on Phase 0
Phase 6:  reqwest client      ← depends on Phase 0
Phase 7:  JWT/sessions        ← depends on Phase 1, Phase 2, Phase 4
Phase 8:  Background jobs     ← depends on Phase 1, Phase 4
Phase 9:  Stateless/scaling   ← depends on Phase 4, Phase 5, Phase 7
Phase 10: Observability       ← depends on Phase 0
Phase 11: Operations          ← depends on Phase 0–9
Phase 12: Features            ← depends on Phase 1
Phase 13: MFA                 ← depends on Phase 1, Phase 2, Phase 7
Phase 14: Security tests      ← depends on Phase 0–13
```

**Recommended start order:**
```
go(Phase 0)
┣━ go(Phase 1) ━━━ Phase 2 (parallel)
┃                 ┃
┃        Phase 4 (Redis) needed by Phase 3, 7, 8, 9
┃        ┃
┃        ┗━ Phase 3 ━━ Phase 5 ━━ Phase 6 (parallel) ━━ Phase 7 ━━ Phase 9
┃                                                       ┃
┃                                              Phase 8 (background jobs)
┗━ Phase 10 ━━ Phase 11 ━━ Phase 12 ━━ Phase 13 ━━ Phase 14
```
