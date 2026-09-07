# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2026-09-07

### Added

- `queries/highlights.scm`: standard syntax-highlight captures for YANG
  (keywords, comments, strings, dates, booleans, module/prefix names,
  type/grouping/identity references, data-node and definition names). Zed,
  Neovim/nvim-treesitter and the tree-sitter CLI read this file straight from
  the grammar repo. Regression-guarded by `049_highlights_query.rs`, which
  compiles the query file and asserts every headline capture class fires.
- Regression tests covering the grammar fixes below: `038_max_elements.rs`,
  `039_vendor_symbol_arg.rs`, `040_units_symbol_arg.rs`,
  `041_enum_bare_symbol_name.rs`, `042_default_symbol_arg.rs`,
  `043_escape_sequences.rs`, `044_type_error_localization.rs`,
  `045_text_argument_tolerance.rs`, `046_recovery_localization.rs`,
  `047_key_unique_concat.rs`, and `048_range_length_concat.rs`.
- `max-elements-stmt` accepts the keyword `unbounded`, producing a named
  `unbounded` node (RFC 7950 §7.7.4).

### Changed

- Unknown/vendor extension statements, and the `units`, `enum`, and `default`
  statements, accept bare (unquoted) arguments containing symbols, e.g.
  `m^-X`, `meter^2.second-1`, `n+1`, `00:00:15.0`, and `syslogtypes:local7`.
- Double-quoted strings accept arbitrary backslash escapes (`\*`, `\S`, `\.`,
  …) in addition to the RFC 7950 §6.1.3 set (`\n`, `\t`, `\"`, `\\`);
  real-world modules use more, and pyang tolerates them with a warning.
- `key`, `unique`, `range`, and `length` arguments are treated as opaque quoted
  strings (RFC 7950), so `+` concatenation (e.g. `key "a " + "b"` or
  `range "… | " + "…"`) and trailing whitespace inside the quotes now parse.
- Bad `type`/text arguments and an unexpected token at a list end are now
  localized errors: the parser recovers deterministically instead of collapsing
  the rest of the module into `ERROR`.
- Regenerated `src/grammar.json`, `src/node-types.json`, and `src/parser.c`
  (`unbounded` and additional `string` children appear in the node types).

### Fixed

- Valid documents no longer collapse into a single `ERROR` node: `max-elements
  unbounded`, symbol-bearing bare arguments to unknown statements, `units`,
  `enum` names, and `default`, arbitrary backslash escapes in double-quoted
  strings, and concatenated `key`/`unique`/`range`/`length` arguments. Together
  these clear the IEEE 1906.1 modules, ietf-coms-core, ietf-routing-types,
  ietf-igmp-mld, iana-* registry modules, ietf-netconf-time, ietf-syslog,
  ietf-netconf-acm, ietf-ipfix-psamp, draft ietf-isis, and other
  MIB/registry-derived transcripts from whole-corpus parse errors.

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

[0.4.0]: https://github.com/trislu/tree-sitter-yang/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/trislu/tree-sitter-yang/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/trislu/tree-sitter-yang/compare/v0.1.3...v0.2.0
