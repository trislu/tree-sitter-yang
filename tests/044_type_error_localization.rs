//! Error-localization regression (PHASE 0): a syntax error in one statement
//! must NOT collapse the whole module into a single top-level ERROR. A bad
//! `type` argument (e.g. `type ^bad`) should produce a local ERROR while the
//! surrounding statements still parse (root stays `yang`).

mod test_utils;

use test_utils::str_to_ast;

fn leaf_count(src: &str) -> usize {
    let tree = str_to_ast(src);
    let mut n = 0;
    fn walk(node: tree_sitter::Node, n: &mut usize) {
        if node.kind() == "leaf_stmt" {
            *n += 1;
        }
        let mut c = node.child_count();
        while c > 0 {
            c -= 1;
            if let Some(ch) = node.child(c as u32) {
                walk(ch, n);
            }
        }
    }
    walk(tree.root_node(), &mut n);
    n
}

#[test]
fn test_type_error_stays_localized() {
    let src = r#"module m {
  namespace "urn:m";
  prefix m;
  container c {
    leaf a { type string; }
    leaf b { type ^bad }
    leaf d { type string; }
  }
}"#;
    let tree = str_to_ast(src);
    assert_eq!(
        tree.root_node().kind(),
        "yang",
        "a mid-module type typo must not collapse the whole module"
    );
    assert!(tree.root_node().has_error(), "local error expected");
    // Sibling statements before and after the bad one are still in the tree.
    assert_eq!(leaf_count(src), 3);
}

#[test]
fn test_prefixed_type_still_parses() {
    let src = r#"module m {
  namespace "urn:m";
  prefix m;
  leaf t { type iana:if-type; }
}"#;
    let tree = str_to_ast(src);
    assert!(!tree.root_node().has_error(), "prefixed type must parse");
}
