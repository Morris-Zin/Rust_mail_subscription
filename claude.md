# zero2prod

Rust email subscription service built with Actix-web, following the "Zero To Production In Rust" approach.

## Build & Run

```bash
cargo build            # Debug build
cargo build --release  # Release build
cargo run              # Run the server (binds to 127.0.0.1:8000)
```

## Test

```bash
cargo test             # Run all tests
```

## Lint & Format

```bash
cargo fmt --check      # Check formatting
cargo fmt              # Auto-format code
cargo clippy -- -D warnings  # Lint with warnings as errors
```

## Project Structure

- `src/main.rs` — Binary entry point; calls `run()` from the library
- `src/lib.rs` — Library crate containing the HTTP server setup and route handlers
- `.cargo/config.toml` — Cargo config (LLD linker flags for Windows targets)
- `.github/workflows/general.yml` — CI: test, fmt, clippy, code coverage (tarpaulin)
- `.github/workflows/audit.yml` — Daily security audit via `cargo deny`

## Architecture

The project uses a binary+library split pattern. `lib.rs` exposes a `run()` function that configures and starts the Actix-web `HttpServer`. `main.rs` is a thin wrapper that calls `run()`. This separation allows integration tests to start the server programmatically.

## Key Dependencies

- **actix-web 4** — Async web framework
- **tokio** — Async runtime (multi-thread + macros features)
- **reqwest** (dev) — HTTP client for integration tests

## CI

GitHub Actions runs on every push and PR:

- `cargo test`
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- Code coverage via `cargo tarpaulin`
- Daily security audit via `cargo deny check advisories`
