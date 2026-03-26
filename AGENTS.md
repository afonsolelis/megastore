# Repository Guidelines

## Project Structure & Module Organization

This repository is a small Rust CLI application for product search over JSON-backed graph data.

- `src/main.rs`: entry point, CLI loop, and result rendering.
- `src/file_loader.rs`: loads product files from `data/`.
- `src/models.rs`: shared data models deserialized with `serde`.
- `src/search.rs`: search logic and unit tests.
- `data/`: sample product documents in JSON format.
- `Cargo.toml`: crate metadata and dependencies.

Keep new logic in focused modules instead of growing `main.rs`. If a feature has its own behavior and tests, give it its own file under `src/`.

## Build, Test, and Development Commands

- `cargo build`: compile the project.
- `cargo run`: start the interactive CLI locally.
- `cargo test`: run the unit test suite.
- `cargo fmt`: format Rust source files.
- `cargo check`: run a fast compile check without producing a binary.

Example:

```bash
cargo run
```

Then use commands like `search lente` or `exit`.

## Coding Style & Naming Conventions

Use standard Rust formatting with 4-space indentation and run `cargo fmt` before submitting changes. Follow Rust naming conventions:

- `snake_case` for functions, modules, and variables.
- `CamelCase` for structs and enums.
- Keep modules small and responsibilities explicit.

Prefer descriptive names such as `load_products` and `search_products`. Avoid embedding unrelated logic directly in `main.rs`.

## Testing Guidelines

Tests currently use Rust’s built-in test framework with `#[test]` functions in `src/search.rs`. Name tests by behavior, for example `test_search_case_insensitivity`.

Add tests for any new search behavior, parsing rules, or CLI-facing logic that can be isolated. Run:

```bash
cargo test
```

before opening a pull request.

## Commit & Pull Request Guidelines

Git history currently uses short conventional-style messages such as `feat: Add initial version of the graph search engine`. Follow that pattern:

- `feat: ...`
- `fix: ...`
- `refactor: ...`

Pull requests should include a short summary, the reason for the change, and notes on how it was validated. If behavior changes, include example commands or sample output. For data model changes, mention any required updates under `data/`.
