# Megastore - Graph Search

## Project Overview

A Rust-based product search engine that loads product data from JSON files and provides a command-line interface for searching products by name or description. Products are modeled as nodes in a graph with relationships (edges) to other products, enabling features like "related products" and "bought together" suggestions.

### Architecture

The project is organized into four modules:

| Module | Description |
|--------|-------------|
| `main.rs` | CLI entry point with interactive REPL loop (`search`, `exit` commands) |
| `models.rs` | Data structures: `Product`, `ProductData`, `Edge` (serde deserialization) |
| `file_loader.rs` | Loads product JSON files from a directory into a `HashMap<i32, Product>` |
| `search.rs` | Case-insensitive search by product name or description |

### Data Model

```
Product
├── id: i32
├── name: String
├── description: String
├── data: ProductData { price: f64, category: String }
└── edges: Vec<Edge> { to_id: i32, relationship: String }
```

## Building and Running

### Prerequisites

- Rust toolchain (edition 2024)

### Commands

```bash
# Build the project
cargo build

# Run the application
cargo run

# Run tests
cargo test

# Check for compilation errors
cargo check
```

### Usage

Once running, the application provides an interactive prompt:

```
Carregando produtos...
X produtos carregados. Motor de busca pronto.
> search laptop
ID: 1 | Nome: Laptop | Preço: 1200.00
  -> Relações:
     - COMPATIVEL_COM: Mouse (ID: 2)
> exit
Saindo...
```

## Development Conventions

- **Language**: Portuguese (PT-BR) for user-facing messages; English for code identifiers
- **Error Handling**: Uses `Result<T, Box<dyn std::error::Error>>` for file/parse operations
- **Testing**: Unit tests in `search.rs` demonstrate case-insensitivity and edge cases
- **Data Format**: Products stored as JSON files in `./data/` directory (one file per product)

## File Structure

```
megastore/
├── Cargo.toml          # Project manifest (serde, serde_json dependencies)
├── readme.md           # Minimal readme ("Olá")
├── data/               # Product JSON files
│   ├── 123.json
│   └── 888.json
└── src/
    ├── main.rs         # CLI entry point
    ├── models.rs       # Data structures
    ├── file_loader.rs  # JSON file loading
    └── search.rs       # Search logic + tests
```
