<!-- Copyright 2026 Salesforce, Inc. All rights reserved. -->
# AGENT.md

Context for AI coding agents (Claude Code, Cursor, etc.) working in a project scaffolded from this template.

## What this repository is

A custom policy for [MuleSoft Flex Gateway](https://docs.mulesoft.com/gateway/) built with the [Policy Development Kit (PDK)](https://docs.mulesoft.com/pdk/latest/). The policy is written in Rust, compiled to WebAssembly (target `wasm32-wasip1`), and runs as a [proxy-wasm](https://github.com/proxy-wasm/spec) filter inside Flex Gateway. PDK abstracts the asynchronous proxy-wasm event model into a simpler `async`/`await` API.

For step-by-step authoring guidance (scaffolding, implementing, testing), use the corresponding PDK skills available in your agent environment. This file only describes the layout and constraints of an already-scaffolded project.

## Project layout

```
.
├── definition/gcl.yaml          # Policy schema (configurable properties) — source of truth
├── src/
│   ├── lib.rs                   # Filter logic — edit here
│   └── generated/config.rs      # AUTO-GENERATED from gcl.yaml — do NOT edit by hand
├── tests/
│   ├── requests.rs              # Integration tests (pdk-test, requires Docker)
│   ├── common/mod.rs
│   └── config/registration.yaml # Required Flex Gateway registration (Local Mode)
├── playground/                  # Local Flex Gateway + sample backend for manual runs
├── Cargo.toml
└── Makefile
```

Edit `definition/gcl.yaml` to declare configurable properties, edit `src/lib.rs` for filter logic, edit `tests/requests.rs` for integration tests. Everything else is generated or boilerplate.

## Build commands

| Command                  | Description                                                       |
|--------------------------|-------------------------------------------------------------------|
| `make setup`             | Install `cargo-anypoint` and `cargo-llvm-cov`                     |
| `make build-asset-files` | Regenerate `src/generated/config.rs` from `definition/gcl.yaml`   |
| `make build`             | Compile the policy to WASM                                        |
| `make test`              | Run integration tests (requires Docker)                           |
| `make run`               | Start the playground via `docker compose`                         |
| `make publish`           | Publish a development version to Exchange                         |
| `make release`           | Publish a release version to Exchange                             |

If `CARGO_TARGET_DIR` is set, unset it before running `make`.

## Restrictions

These are non-negotiable for any policy code in this project.

### Forbidden Cargo features

Do not enable any of the following — experimental or low-level, may break without notice between releases, not part of the PDK public contract:

- `experimental_*` (any feature with this prefix)
- `ll`
- `script_stream`
- `enable_stop_iteration`
- `experimental_metrics`
- `experimental_datastorage_formats`
- `experimental_disable_body_limit_check`

### proxy-wasm runtime

PDK runs on proxy-wasm — single-threaded inside the policy runtime. Code that compiles fine on a desktop target can still be rejected at runtime if it violates these:

- No multithreading; no `Arc`, `Mutex`, `RwLock`, or other cross-thread synchronization primitives.
- No `block_on`, no synchronous waits, no blocking I/O.
- No full async runtimes (Tokio multi-thread, blocking features, etc.). Use the async model PDK exposes.
- Use `thread_local!` when process-wide state is genuinely required.

### Coding rules

- **Rust toolchain:** stable only. Nightly features, `rustc` flags, or `rustup` overrides selecting nightly are not allowed.
- **`unsafe`:** forbidden in policy code.
- **`.unwrap()`:** avoid in production code.
- **`src/generated/config.rs` is auto-generated** — never edit by hand, run `make build-asset-files` instead.
- **License header:** every source file starts with `// Copyright YYYY Salesforce, Inc. All rights reserved.`

## Common pitfalls

These cause real bugs in production policies. Watch for them before writing code.

- **State machine consumes ownership.** `RequestState` → `HeadersState` → `BodyState` (and the response-side equivalents) each transition consumes the previous state. Read everything you need from headers before transitioning to the body — you cannot go back.
- **Check `contains_body()` before reading the body.** Calling `.body()` on a request without a body (GET, HEAD, empty POST) panics under proxy-wasm.
- **`gcl.yaml` defaults arrive pre-filled.** Flex Gateway applies `default` values before the configuration bytes reach the policy, so a `required: true` property with a `default` is never absent at parse time. Do not write code that branches on "missing required field".
- **Empty configuration bytes are possible.** Check `is_empty()` on the `Configuration` payload before calling `serde_json::from_slice` — an empty buffer fails parsing and the policy never launches.
- **Always include the raw config bytes in parse-error logs** (via `String::from_utf8_lossy`). Without them the operator cannot debug why the policy refused to load.
- **`Flow::Break(response)` rejects, `Flow::Continue(())` allows.** Inverting these is a security hole: an auth filter that returns `Continue` on failure passes the unauthenticated request to the upstream.
- **Response filter must handle `RequestData::Break`.** If the request was rejected by an earlier filter, the response filter receives `Break(response)`, not `Continue(data)`. `.unwrap()` on a `Break` will crash.
- **Header names are case-insensitive.** Lowercase both sides before comparing (`name.to_ascii_lowercase()`); production policies do this consistently.
- **Decide explicitly how `HttpClient` errors are handled** (timeout, DNS failure, upstream 5xx). Fail-open vs fail-closed is a security decision — surface it in the policy config, do not silently swallow `.await` errors.

## PDK ecosystem map

Most policies are built from a small set of core APIs plus 0–2 extension libraries. Reach for an existing library before writing your own primitive.

### Core APIs (frequency in production policies)

| Module | Purpose |
|--------|---------|
| `pdk::hl` | High-level async framework: `Flow<T>`, `RequestState`/`ResponseState`, `HttpClient`, `Service`, `Response`, timers. Entry point for every policy. |
| `pdk::logger` | Structured logging with policy and request context auto-injected. Use `debug!`, `info!`, `warn!`, `error!`. |
| `pdk::policy_violation` | Mark expected, policy-driven request rejections (auth fail, rate limit, validation error). Drives audit and observability. |
| `pdk::authentication` | Share extracted credentials, tokens, and principals between filters in the same request. |
| `pdk::script` | Evaluate PEL (Policy Expression Language) expressions provided by the policy config — for dynamic selection logic, do not hard-code. |
| `pdk::metadata` | Read API and request metadata injected by the platform (SLA tier, custom metadata). |

### Extension libraries (most-used first)

| Library | Use when… |
|---------|----------|
| `pdk::contracts` | Validating client credentials against Anypoint Platform contracts (with local caching). |
| `pdk::rl` | Enforcing per-client / per-tier request rate caps (token bucket, local or distributed). |
| `pdk::jwt` | Parsing and validating JWTs and extracting claims. Never re-implement signature validation. |
| `pdk::token_introspection` | Validating opaque OAuth2 tokens or checking scopes via introspection endpoint. |
| `pdk::ip_filter` | Allow- or block-listing by IP/CIDR. Do not parse CIDRs with regex. |
| `pdk::data_storage` | Durable, optionally distributed key-value store with CAS semantics. |

Also available but rarely used in current policies: `pdk::cache`, `pdk::cors`, `pdk::json_validator`, `pdk::xml_validator`, `pdk::ldap`, `pdk::lock`, `pdk::spike_control`, `pdk::metrics`. Prefer them over hand-rolled equivalents.

### `pdk::cache` vs `pdk::data_storage`

Both are key-value stores; they solve different problems.

- **`pdk::cache`** — in-memory, FIFO eviction, fast. Use for hot-path caching where data can be lost on restart (HTTP response caching, JWKS caching, short-lived token lookups).
- **`pdk::data_storage`** — local or clustered backend, persistent, supports compare-and-swap. Use for coordinated state that must survive restarts or be shared across instances (distributed quotas, spike-control state, cross-policy coordination).

## Resources

- PDK documentation — https://docs.mulesoft.com/pdk/latest/
- Flex Gateway documentation — https://docs.mulesoft.com/gateway/
- Public policy examples — https://github.com/mulesoft/pdk-custom-policy-examples
- This template — https://github.com/mulesoft/microgateway-custom-policy-template
