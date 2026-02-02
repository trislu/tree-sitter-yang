# tree-sitter-yang

[![Rust CI](https://github.com/trislu/tree-sitter-yang/actions/workflows/rust.yml/badge.svg)](https://github.com/trislu/tree-sitter-yang/actions/workflows/rust.yml)
[![Cargo Publish](https://github.com/trislu/tree-sitter-yang/actions/workflows/publish.yml/badge.svg)](https://github.com/trislu/tree-sitter-yang/actions/workflows/publish.yml)
[![Latest Version](https://img.shields.io/crates/v/tree-sitter-yang.svg)](https://crates.io/crates/tree-sitter-yang)
[![License](https://img.shields.io/crates/l/tree-sitter-yang.svg)](LICENSE)

## Introduction

A robust [YANG](https://www.rfc-editor.org/rfc/rfc6020) parser library built on [tree-sitter](https://tree-sitter.github.io/tree-sitter/).

## Supported YANG RFCs
This library implements parsing for the official YANG specifications:
- [RFC 6020](https://www.rfc-editor.org/rfc/rfc6020): YANG Version 1.0
- [RFC 7950](https://www.rfc-editor.org/rfc/rfc7950): YANG Version 1.1 (latest core spec)

## Installation

### Rust
Run `cargo add tree-sitter-yang` or add below content to your `Cargo.toml`:
```toml
[dependencies]
tree-sitter = "0.20"
tree-sitter-yang = { git = "https://github.com/trislu/tree-sitter-yang" }
```

### Other Languages
Tree-sitter supports bindings for Node.js, Python, C, and more. See the [Tree-sitter Language Bindings](https://tree-sitter.github.io/tree-sitter/bindings) docs for setup instructions.

## Usage (Rust Example)
```rust
use tree_sitter::Parser;

fn main() {
    // Create a new parser
    let mut parser = Parser::new();
    let language = tree_sitter_yang::LANGUAGE;
    parser.set_language(&language.into()).expect("Error loading YANG language");

    // Parse a YANG snippet
    let yang_code = r#"
        module example {
            namespace "urn:example:yang";
            prefix "ex";
            
            leaf hostname {
                type string;
                description "Device hostname";
            }
        }
    "#;

    let tree = parser.parse(yang_code, None).unwrap();
    let root_node = tree.root_node();

    // Print the s-expression of the AST
    let sexp = tree_sitter::format_sexp(&root_node().to_sexp(), 0);
    println!("--- The formatted s-expression ---\n{}", sexp);
}
```

## Development

### The Grammar
1. Install _tree-sitter-cli_: `cargo install tree-sitter-cli`
2. Generate the parser code: `tree-sitter generate --abi=14`

### The Rust binding
```bash
# Build
cargo build --feautes dev
# Rust tests
cargo test
```

## Contributing
Contributions are welcome! Here’s how to help (or see [more](CONTRIBUTING.md) details):
- Report bugs or feature requests via [Issues](https://github.com/trislu/tree-sitter-yang/issues)
- Submit PRs for grammar fixes, performance improvements, or new features
- Ensure all tests pass before submitting PRs


## License
This project is licensed under the [MIT License](LICENSE).