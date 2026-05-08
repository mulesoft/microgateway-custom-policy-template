<!-- Copyright 2026 Salesforce, Inc. All rights reserved. -->
# CLAUDE.md

This file provides guidance to [Claude Code](https://claude.com/claude-code) when working with code in this repository.

## Project Overview

This project is a custom policy for [MuleSoft Flex Gateway](https://docs.mulesoft.com/gateway/) built with the [Policy Development Kit (PDK)](https://docs.mulesoft.com/pdk/latest/). The policy is written in Rust, compiled to WebAssembly (target `wasm32-wasip1`), and runs as a [proxy-wasm](https://github.com/proxy-wasm/spec) filter inside Flex Gateway. PDK abstracts the asynchronous proxy-wasm event model into a simpler `async`/`await` API.

## Prerequisites

### Tooling

- **Rust** 1.88.0+ (`rustup update`)
- **WASI target**: `rustup target add wasm32-wasip1`
- **cargo-generate** 0.22.0: `cargo install --locked cargo-generate@0.22.0`
- **anypoint-cli-v4** 1.4.4+ with the `anypoint-pdk-plugin`
- **make**
- **Docker** (required to run integration tests and the playground)

Run `make setup` after cloning to install the remaining build tooling (`cargo-anypoint`, `cargo-llvm-cov`).

### Recommended Claude Code skills

For deeper guidance, install the public PDK skill set from [mulesoft-emu/pdk-skills-scripts](https://github.com/mulesoft-emu/pdk-skills-scripts). The skills below are referenced throughout this file:

- **pdk-common** — shared PDK Rust API conventions (apply first).
- **pdk-policy-development** — policy structure, gcl.yaml, integration tests.
- **pdk-unit-testing** — unit tests with the `pdk-unit` framework.

Two installation methods are supported:

```bash
# Method 1 — Claude plugin marketplace
git clone https://github.com/mulesoft-emu/pdk-skills-scripts.git
# Inside Claude Code:
/plugin marketplace add /path/to/pdk-skills-scripts
/reload-plugins

# Method 2 — symlink setup script
git clone https://github.com/mulesoft-emu/pdk-skills-scripts.git
cd pdk-skills-scripts
./setup.sh
```

The skills are optional. Everything in this `CLAUDE.md` is self-contained, but the skills add detailed checklists and patterns.

## Build & Development Commands

| Command                  | Description                                                       |
|--------------------------|-------------------------------------------------------------------|
| `make setup`             | Install `cargo-anypoint` and `cargo-llvm-cov`                     |
| `make build-asset-files` | Fetch policy definition and regenerate `src/generated/config.rs`  |
| `make build`             | Compile the policy to WASM and produce the implementation asset   |
| `make test`              | Build and run integration tests                                   |
| `make test-coverage`     | Run tests with `cargo-llvm-cov` (`FORMAT=json\|html` optional)    |
| `make run`               | Build and start the playground via `docker compose`               |
| `make publish`           | Publish a development version to Exchange                         |
| `make release`           | Publish a release version to Exchange                             |

If `CARGO_TARGET_DIR` is set in your environment, unset it before running `make`:

```bash
unset CARGO_TARGET_DIR && make build
```

## Project Structure

```
.
├── definition/gcl.yaml          # Policy schema (configurable properties)
├── src/
│   ├── lib.rs                   # Policy entrypoint and filter logic — edit here
│   └── generated/
│       ├── mod.rs
│       └── config.rs            # AUTO-GENERATED from gcl.yaml — do NOT edit by hand
├── tests/
│   ├── requests.rs              # Integration tests (pdk-test, requires Docker)
│   ├── common/mod.rs            # Test constants
│   └── config/
│       └── registration.yaml    # Required Flex Gateway registration (Local Mode)
├── playground/
│   ├── docker-compose.yaml      # Local Flex Gateway + sample backend
│   └── config/
│       ├── api.yaml             # Sample policy configuration
│       ├── registration.yaml    # Same registration used for `make run`
│       └── custom-policies/     # Built policy artifacts (auto-populated)
├── Cargo.toml
├── Makefile
└── .project.yaml
```

## Core PDK Concepts

A minimal policy looks like this:

```rust
use anyhow::{anyhow, Result};
use pdk::hl::*;
use pdk::logger;

use crate::generated::config::Config;

async fn request_filter(request_state: RequestState, config: &Config) {
    let headers_state = request_state.into_headers_state().await;
    let token = headers_state.handler().header("Token").unwrap_or_default();
    logger::info!("Header value: {token}");
    headers_state
        .handler()
        .set_header("x-added", &config.string_property);
}

#[entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration) -> Result<()> {
    let config: Config = serde_json::from_slice(&bytes).map_err(|err| {
        anyhow!(
            "Failed to parse configuration '{}'. Cause: {}",
            String::from_utf8_lossy(&bytes),
            err
        )
    })?;
    let filter = on_request(|rs| request_filter(rs, &config));
    launcher.launch(filter).await?;
    Ok(())
}
```

Key building blocks:

- `#[entrypoint]` — marks the `configure` function as the policy entry point.
- `Launcher` — registers `on_request` and/or `on_response` filters.
- `Flow<T>` — returned from filter logic to control flow (`Flow::Continue(_)` or `Flow::Break(response)`).
- **Request state machine** — `RequestState` → `HeadersState` → `BodyStreamState`. Each transition **consumes** the previous state, so extract everything you need from headers before moving to the body.
- `Configuration(bytes)` — raw configuration bytes from Flex Gateway, parsed into the generated `Config` struct.

For policy structure, `gcl.yaml` schema details, response filters, body streaming, logging, violations, HTTP upstream calls, JWT/OAuth/CORS/contracts/rate-limiting/storage/cache/IP-filter helpers, and DataWeave expressions, see the **pdk-policy-development** skill or [the official PDK docs](https://docs.mulesoft.com/pdk/latest/).

## 🚫 Forbidden Features

Do not use the following Cargo features or modules in production policy code. They are experimental or low-level, may break without notice between releases, and are not part of the PDK public contract:

- `experimental_*` (any feature with this prefix)
- `ll`
- `script_stream`
- `enable_stop_iteration`
- `experimental_metrics`
- `experimental_datastorage_formats`
- `experimental_disable_body_limit_check`

If you believe one of these is required, raise it with the PDK team before adding it to your policy.

## proxy-wasm Runtime Constraints

PDK runs on **proxy-wasm**, which is effectively single-threaded inside the policy runtime. Code that compiles fine on a desktop target can still misbehave or be rejected at runtime if it violates these constraints:

- **No multithreading.** Do not assume threads, thread pools, or parallel execution.
- **Avoid** `Arc`, `Mutex`, `RwLock`, and other cross-thread synchronization primitives.
- **Do not block the host.** No `block_on`, no synchronous waits, no blocking I/O. The Wasm VM cannot block the host while it has not polled.
- **No full async runtimes.** Do not pull in Tokio multi-thread, blocking features, or any runtime incompatible with proxy-wasm. Use the async model PDK exposes.
- **Use `thread_local!`** when you genuinely need process-wide state.
- **Standard network/filesystem crates do not work** under `wasm32-wasip1`. Use PDK-provided APIs (e.g. `HttpClient`) for I/O.

## Coding Conventions

- **Rust toolchain:** stable only. Nightly features, `rustc` flags, or `rustup` overrides selecting nightly are not allowed.
- **`.unwrap()`:** avoid in production code. If an unwrap is provably safe by construction, leave a short comment explaining why it cannot panic.
- **`unsafe`:** forbidden in policy code.
- **Builders:** new builder-style APIs use `.property_name()` setters, not `.with_property_name()` (the legacy `with_*` setters in `pdk-unit` are an exception kept for backwards compatibility).
- **Public errors:** mark public error enums with `#[non_exhaustive]`; variant names should not end with the `Error` suffix.
- **License headers:** every source file in the repository must start with `// Copyright YYYY Salesforce, Inc. All rights reserved.`

For the full conventions checklist (accessors, re-exports, `experimental_*` documentation rules, etc.), see the **pdk-common** skill.

## Testing

### Unit tests (pdk-unit)

Add unit tests inside a `#[cfg(test)]` module **at the end of the same file** that contains the logic under test (typically `src/lib.rs`). The framework stubs proxy-wasm so tests run in-process without Envoy or a real WebAssembly host.

```rust
#[cfg(test)]
mod test {
    use pdk_unit::{UnitTestBuilder, UnitHttpRequest, UnitHttpResponse, UnitHttpMessage};
    use serde_json::json;

    #[test]
    fn adds_custom_header() {
        let mut tester = UnitTestBuilder::default()
            .with_backend(UnitHttpResponse::new(200))
            .with_config(json!({"stringProperty": "custom"}).to_string())
            .with_entrypoint(super::configure);

        let response = tester.request_full(UnitHttpRequest::get());
        assert_eq!(response.status_code(), 200);
    }
}
```

If `configure` accepts a `Clock` parameter, call `tester.sleep(...)` after `with_entrypoint(...)` so async initialization completes before the first request.

For mocking upstreams (`TraceBackend`), identity providers, contracts/SLAs, gRPC backends, and DataWeave expressions, see the **pdk-unit-testing** skill.

### Integration tests (pdk-test)

Integration tests in `tests/requests.rs` spin up real Flex Gateway and backend containers via Docker. They require:

- A working Docker daemon.
- `tests/config/registration.yaml` — register Flex Gateway in Local Mode to generate this file. The same file must also exist under `playground/config/` for `make run`.

Run them with `make test`.

## Common Gotchas

1. **`src/generated/config.rs` is auto-generated.** `make build-asset-files` overwrites it from `definition/gcl.yaml`. Edit `gcl.yaml`, not the generated file.
2. **`registration.yaml` is required** for both integration tests and the playground. Without it, Flex Gateway will not start in Local Mode.
3. **The state machine consumes state.** `into_headers_state()` consumes `RequestState`; `into_body_stream_state()` consumes `HeadersState`. Read everything you need from the previous state before transitioning.
4. **URL paths and query strings are percent-encoded.** Decode with `percent_encoding::percent_decode_str` before pattern matching.
5. **`gcl.yaml` defaults are applied by Flex Gateway** before configuration bytes reach the policy, so `required` fields with `default` values arrive pre-filled.
6. **Integration tests are slow** because each one starts Docker containers. Use unique `hostname` values per test to avoid collisions and keep the test count reasonable.

## Resources

- PDK documentation — https://docs.mulesoft.com/pdk/latest/
- Flex Gateway documentation — https://docs.mulesoft.com/gateway/
- Public policy examples — https://github.com/mulesoft/pdk-custom-policy-examples
- This template — https://github.com/mulesoft/microgateway-custom-policy-template
- PDK Claude Code skills — https://github.com/mulesoft-emu/pdk-skills-scripts
