# TodoMVC - Leptos

A full-stack [TodoMVC](https://todomvc.com) implementation built with [Leptos](https://leptos.dev) 0.8, Axum 0.8, and SQLite.

## Prerequisites

- **Rust** (stable toolchain, 1.80+)
- **wasm32-unknown-unknown** target: `rustup target add wasm32-unknown-unknown`
- **cargo-leptos**: `cargo install cargo-leptos`

## Quick Start

```bash
# Development mode with hot-reload
cargo leptos watch

# Production build
cargo leptos build --release
```

The app listens on **http://0.0.0.0:8080** by default.

## Configuration

| Variable | Default | Description |
|---|---|---|
| `DATABASE_URL` | `sqlite:todos.db?mode=rwc` | SQLite connection string |

The database is created automatically on first run, and migrations run at startup.

## Build Commands

```bash
# Development build
cargo leptos build

# Release build
cargo leptos build --release

# Run tests
cargo test --features ssr

# Build SSR server only
cargo build --features ssr --no-default-features

# Build WASM client only
cargo build --target wasm32-unknown-unknown --features hydrate --no-default-features
```

## Project Structure

```
Cargo.toml                  # Dependencies + cargo-leptos config
rust-toolchain.toml         # Rust toolchain settings
migrations/                 # SQLx migrations (auto-run at startup)
style/
  main.css                  # TodoMVC CSS (app + common styles)
src/
  main.rs                   # Axum server entry point (SSR)
  lib.rs                    # WASM hydrate entry point + module exports
  app.rs                    # Root App component, HTML shell, routing
  db.rs                     # SQLite pool initialization + migrations
  model.rs                  # Todo struct + SQLx CRUD queries
  server_fns.rs             # Leptos server functions (add, toggle, delete, etc.)
  components/
    mod.rs                  # Component exports + Filter enum
    todo_input.rs           # Header with new-todo input
    todo_list.rs            # Main section with toggle-all + filtered todo list
    todo_item.rs            # Single todo row (toggle, edit, destroy)
    footer.rs               # Item count, filter links, clear completed
tests/
  integration_test.rs       # Integration tests against real SQLite
```

## Architecture

The app uses Leptos's SSR + hydration model via `cargo-leptos`:

- **Server** (`ssr` feature): Axum serves pre-rendered HTML, handles server function RPCs, and serves static assets. The SQLite pool is shared via Axum `Extension`.
- **Client** (`hydrate` feature): WASM bundle hydrates the server-rendered HTML for interactivity. Server functions become HTTP requests to the server.
- **Reactivity**: A `Resource` fetches todos from the server. Mutations call server functions then `.refetch()` the resource to update the UI.
- **Filtering**: Client-side via URL hash (`#/`, `#/active`, `#/completed`).

## License

MIT
