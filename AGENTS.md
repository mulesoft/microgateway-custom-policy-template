<!-- Copyright 2026 Salesforce, Inc. All rights reserved. -->
# AGENTS.md

Context for AI coding agents (Claude Code, Cursor, Codex, Aider, etc.) working in a project scaffolded from this template. Follows the [agents.md](https://agents.md) convention.

## What this repository is

A custom policy for [MuleSoft Flex Gateway](https://docs.mulesoft.com/gateway/) built with the [Policy Development Kit (PDK)](https://docs.mulesoft.com/pdk/latest/). The policy is written in Rust, compiled to WebAssembly (target `wasm32-wasip1`), and runs as a [proxy-wasm](https://github.com/proxy-wasm/spec) filter inside Flex Gateway. PDK abstracts the asynchronous proxy-wasm event model into a simpler `async`/`await` API.

## Project layout

```
.
├── definition/gcl.yaml          # Policy schema — declares configurable properties
├── src/
│   ├── lib.rs                   # Filter logic — edit here
│   └── generated/config.rs      # AUTO-GENERATED from definition/gcl.yaml — do NOT edit by hand
├── tests/
│   ├── requests.rs              # Integration tests (pdk-test, requires Docker)
│   ├── common/mod.rs
│   └── config/
│       └── note.txt             # Drop your registration.yaml here per these instructions
├── playground/                  # `make run` artifacts: local Flex Gateway + sample backend
│   ├── docker-compose.yaml      # Spins up Flex Gateway and a backend container
│   └── config/
│       ├── api.yaml             # Sample API + policy config consumed at runtime — edit to test
│       ├── logging.yaml
│       └── custom-policies/     # Your built .wasm artifacts land here (gitignored)
├── Cargo.toml
└── Makefile
```

Edit `definition/gcl.yaml` to change the configurable properties, `src/lib.rs` for filter logic, and `tests/requests.rs` for integration tests. Everything else is generated or boilerplate.

## proxy-wasm runtime

PDK runs on proxy-wasm — single-threaded inside the policy runtime. Code that compiles fine on a desktop target can still be rejected at runtime if it violates these:

- No multithreading; no `Arc`, `Mutex`, `RwLock`, or other cross-thread synchronization primitives.
- No `block_on`, no synchronous waits, no blocking I/O.
- No full async runtimes (Tokio multi-thread, blocking features, etc.). Use the async model PDK exposes.
- Use `thread_local!` when process-wide state is genuinely required.

## Coding rules

- **Rust toolchain:** stable only. Nightly features, `rustc` flags, or `rustup` overrides selecting nightly are not allowed.
- **`unsafe`:** forbidden in policy code.
- **`.unwrap()`:** avoid in production code.
- **`src/generated/config.rs` is auto-generated** from the policy definition — never edit by hand; regenerate via the project's build tooling.
- **License header:** every source file starts with `// Copyright YYYY Salesforce, Inc. All rights reserved.`

## Common pitfalls

These cause real bugs in production policies. Watch for them before writing code.

- **State machine consumes ownership.** `RequestState` → `RequestHeadersState` → `RequestBodyState` (and the response-side equivalents) each transition consumes the previous state. Read everything you need from headers before transitioning to the body — you cannot go back.
- **Check `contains_body()` before reading or writing the body.** On a bodyless request (GET, HEAD, empty POST) `.body()` returns an empty buffer, and writes to it will not reach upstream — you can't add a body that wasn't there in the first place.
- **Definition defaults arrive pre-filled.** Flex Gateway applies `default` values from the policy definition before the configuration bytes reach the policy, so a `required: true` property with a `default` is never absent at parse time. Do not write code that branches on "missing required field".
- **Empty configuration bytes are possible.** Check `is_empty()` on the `Configuration` payload before calling `serde_json::from_slice` — an empty buffer fails parsing and the policy never launches.
- **Always include the raw config bytes in parse-error logs** (via `String::from_utf8_lossy`). Without them the operator cannot debug why the policy refused to load.
- **`Flow::Break(response)` rejects, `Flow::Continue(())` allows.** Inverting these is a security hole: an auth filter that returns `Continue` on failure passes the unauthenticated request to the upstream.
- **Response filter must handle `RequestData::Break`.** If the request was rejected by an earlier filter, the response filter receives `Break(response)`, not `Continue(data)`. `.unwrap()` on a `Break` will crash.
- **Header names are case-insensitive.** Lowercase both sides before comparing (`name.to_ascii_lowercase()`); production policies do this consistently.
- **Decide explicitly how `HttpClient` errors are handled** (timeout, DNS failure, upstream 5xx). Fail-open vs fail-closed is a security decision — surface it in the policy config, do not silently swallow `.await` errors.

## Resources

- PDK documentation — https://docs.mulesoft.com/pdk/latest/
- Flex Gateway documentation — https://docs.mulesoft.com/gateway/
- Public policy examples — https://github.com/mulesoft/pdk-custom-policy-examples
