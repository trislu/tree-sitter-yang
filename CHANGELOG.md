# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[0.2.0]: https://github.com/trislu/tree-sitter-yang/compare/v0.1.3...v0.2.0
