# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-09-05

### Added

- Regression tests covering the grammar fixes below:
  `034_rfc7950_concat_quote.rs`, `035_vendor_extension.rs`,
  `036_descendant_nodeid.rs`, and `037_bare_string_args.rs`.
- `identity-stmt` bodies may now contain `if-feature-stmt`.
- `type-stmt` bodies may now contain vendor `unknown-statement`s alongside the
  standard type-body statements.

### Changed

- **Breaking:** removed the external scanner (`src/scanner.c`). `namespace-stmt`
  arguments are now parsed by the grammar itself as either a quoted URI
  (single- or double-quoted, with optional `+` concatenation) or a bare,
  unquoted URI token that may contain `:`. The Rust binding no longer compiles a
  scanner (`bindings/rust/build.rs`).
- **Breaking:** quoted arguments are now kept opaque. When written quoted
  (single- or `+`-concatenated), the argument of `path`, `refine`,
  `uses-augment`, `augment`, and `deviation` statements is produced as a single
  `quoted_string` node instead of being decoded into nested
  `node_identifier`/`identifier` structure. Unquoted arguments are still parsed
  as structured schema-nodeids. This changes the shape of the syntax tree for
  consumers that walk those arguments.
- **Breaking:** `unknown-statement` bodies `{ ... }` are now consumed as an
  opaque, brace-balanced region rather than being parsed into nested statements.
  Consumers that relied on the internal structure of vendor-extension bodies
  must treat that content as opaque text.
- **Breaking:** `descendant-schema-nodeid` may now be a bare `node-identifier`;
  the `/…` suffix is optional per RFC 7950 (affects `unique`, `refine`,
  `uses-augment`, and `deviation` arguments).
- `if-feature-stmt` and `uses-stmt` arguments may now be quoted (single- or
  `+`-concatenated) or a bare identifier-ref.
- Single-quoted strings no longer interpret `\` escapes, matching RFC 7950
  §6.1.3; a backslash is an ordinary character inside them. Double-quoted
  strings keep `\n`, `\t`, `\"`, and `\\` escapes.
- Bare (unquoted) string arguments are now accepted more broadly, e.g.
  digit-leading `enum` names, slash-containing `units`/`default` values such as
  `Mb/s`, and numeric unknown-statement arguments.
- Regenerated `src/grammar.json`, `src/node-types.json`, and `src/parser.c`.

### Removed

- **Breaking:** `src/scanner.c` and its `externals`/build wiring (see above).

### Fixed

- Valid documents no longer collapse into a single `ERROR` node, including
  `+`-concatenated quoted arguments, unquoted namespace URIs containing `:`,
  backslashes in single-quoted strings, bare `descendant-schema-nodeid`s, and
  vendor extension statements (no-argument bodies, nested bodies, and bodies
  inside `type { ... }`).

## [0.2.0] - 2026-09-04

### Removed

- Rust binding: removed the public `yang` module and everything it re-exported
  (`yang::ast`, `yang::statement`, `yang::statement_generated`, and `yang::token`).
  The crate now exposes only `LANGUAGE` and `NODE_TYPES`; the previous
  statement/AST-helper and token APIs are gone.
- Removed the runtime dependencies that only supported that API: `anyhow`,
  `easy-tree`, `strum`, and `strum_macros`.

### Changed

- Grammar: each statement keyword is now represented as a named node of type
  `*_keyword` (e.g. `module_keyword`, `prefix_keyword`, `namespace_keyword`,
  `container_keyword`) instead of an anonymous token. Every statement node in the
  parse tree now contains a named keyword child:
  - before: `(module_stmt arg: (module_arg_str (identifier)))`
  - after: `(module_stmt (module_keyword) arg: (module_arg_str (identifier)))`

  This is a breaking change for consumers that walk or query the syntax tree, and
  it is reflected in the regenerated `src/node-types.json`.
- Regenerated `src/grammar.json`, `src/node-types.json`, and `src/parser.c`.

### Fixed

- `submodule-stmt` now accepts `body-stmt`s (such as `container`, `leaf`, and
  `list` statements) in addition to the header, linkage, meta, and revision
  statements, matching the RFC grammar. Previously, valid submodules containing
  data-definition statements failed to parse.

[0.3.0]: https://github.com/trislu/tree-sitter-yang/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/trislu/tree-sitter-yang/compare/v0.1.3...v0.2.0
