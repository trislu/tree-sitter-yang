//! PHASE 0 regression (recovery mechanism): deterministic separators
//! (whitespace-only `stmtsep`) with `unknown_stmt` promoted to an explicit
//! item of every statement list lets tree-sitter localize a single unexpected
//! token at a list end instead of collapsing the whole module. It also keeps
//! vendor tolerance where it previously lived only in separators — including
//! vendor statements inside a statement's `{ … }` (newer modules do
//! `organization "…" { amm:enum 1; }`).

mod test_utils;

use test_utils::str_to_ast;

fn leaves(src: &str) -> usize {
    let t = str_to_ast(src);
    let s = t.root_node().to_sexp();
    s.matches("leaf_stmt").count()
}

#[test]
fn test_single_unexpected_token_at_list_end_localizes() {
    let src = r#"module m {
  namespace "urn:m";
  prefix m;
  container c {
    leaf a { type string; }
    bogus
  }
  leaf z { type string; }
}"#;
    let t = str_to_ast(src);
    assert_eq!(t.root_node().kind(), "yang");
    assert!(t.root_node().has_error(), "a local error is expected");
    assert_eq!(leaves(src), 2, "surrounding leaves survive");
}

#[test]
fn test_vendor_statements_in_nonblock_block_parse() {
    // Newer modules attach vendor statements inside e.g. organization's
    // `{ … }`; previously only separator tolerance made this work.
    let src = r#"module m {
  namespace "urn:m";
  prefix m;
  organization "IETF" {
    amm:enum 1;
  }
  contact "x" {
    amm:enum 2;
  }
  leaf z { type string; }
}"#;
    let t = str_to_ast(src);
    assert!(!t.root_node().has_error(), "vendor block must parse");
    assert_eq!(leaves(src), 1);
}
