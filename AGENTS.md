# tree-sitter-yang — repository instructions

A YANG grammar for [tree-sitter](https://tree-sitter.github.io/tree-sitter/)
(RFC 7950 §14 faithful, with vendor-extension tolerance), published as a Rust
crate that `yrepo` consumes.

## Layout facts

- `grammar.js` — the **single source of truth** (top declares token strings as
  `const … = 'keyword'`, e.g. `const add_arg = 'add'`).
- `src/parser.c`, `src/grammar.json`, `src/node-types.json`, `src/tree_sitter/`
  — **generated** by `tree-sitter generate`. Never hand-edit them.
- `bindings/rust/` — hand-written `lib.rs` + `build.rs` that codegens
  `NodeKind`/etc. from the generated artifacts at compile time. Do not hand-edit
  generated binding code.
- Tests are Rust integration tests under `tests/` (numbered `0NN_<topic>.rs`,
  sharing `test_utils.rs`); there is no `test/corpus`.

## Changing the grammar

Follow the `parser-regen` skill (canonical copy:
`.agents/skills/parser-regen/SKILL.md`):

1. Edit `grammar.js` following its existing style.
2. `tree-sitter generate` (CLI 0.26, pinned as dev-dependency) from the repo
   root; diff should touch only generated artifacts under `src/`.
3. `cargo test` here, then verify through the sibling `yrepo` against the real
   corpus — yrepo already carries the `[patch.crates-io]` override, so a local
   fix is live without further wiring.
4. Add a `tests/0NN_*.rs` case for the fix, ideally noting the real-world file
   it fixes.

## Current state

- **Unpublished fix at HEAD** (`5be0850`): `max-elements unbounded;` parses
  again (regression test `038_max_elements.rs`). The crate is still version
  0.3.0 on the working tree — **do not bump versions or publish without the
  user's explicit approval**.
- **Unpublished fixes (committed locally, not released)**:
  - unknown/vendor extension statements accept any bare unquoted argument
    (RFC 7950 unquoted-string), e.g. units like `m^-X` (`039_vendor_symbol_arg.rs`);
  - the `units` statement likewise accepts bare symbol arguments such as
    `meter^2.second-1` (`040_units_symbol_arg.rs`);
  - `enum` names accept bare strings with symbols such as `n+1`
    (`041_enum_bare_symbol_name.rs`) — RFC 7950 enum names are `string`s;
  - the `default` statement accepts bare symbol arguments such as
    `00:00:15.0` or `syslogtypes:local7` (`042_default_symbol_arg.rs`);
  - double-quoted strings accept arbitrary backslash escapes (`\*`, `\S`, `\.`,
    `043_escape_sequences.rs`) — RFC 7950 defines `\n \t \" \\`, real modules
    use more (pyang tolerates with a warning).
  Together they clear the IEEE 1906.1 modules, ietf-coms-core, ietf-routing-types,
  ietf-igmp-mld, iana-* registry modules, ietf-netconf-time, ietf-syslog,
  ietf-netconf-acm, ietf-ipfix-psamp, DRAFT ietf-isis and other MIB/registry
  transcripts.
- **PHASE 0 error-localization set (committed locally, not released)**:
  `044_type_error_localization.rs`, `045_text_argument_tolerance.rs`,
  `046_recovery_localization.rs` — bad `type`/text arguments and a single
  unexpected token at a list end no longer collapse the whole module.
- Whole-corpus parse-errors (~100 after the working-tree fixes, mostly
  `experimental/ietf-extracted-YANG-modules`) are the remaining grammar gaps;
  see the `issue-hunter` skill in `yrepo` for the audit workflow.

## House rules

- Keep grammar fixes minimal and RFC 7950-faithful; check the RFC grammar
  before declaring a construct invalid (cross-check with `pyang` when unsure).
- Keep `cargo fmt` / `clippy` clean.
- Sync README / CHANGELOG / tree-sitter.json metadata on behavior changes.
