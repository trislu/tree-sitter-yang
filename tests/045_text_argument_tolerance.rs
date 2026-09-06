//! PHASE 0 tolerance regression: text arguments (`description`, `organization`,
//! `contact`, `reference`) are strings — RFC 7950 unquoted-string allows
//! nearly any non-delimiter character, so a stray/partial word while editing
//! must not collapse the whole module. Words that don't look like identifiers
//! (e.g. containing `^`, `:`) are consumed as the argument value.

mod test_utils;

use test_utils::str_to_ast;

#[test]
fn test_text_argument_symbol_words_do_not_collapse() {
    for src in [
        r#"module m { namespace "urn:m"; prefix m; leaf a { type string; description ^^ ; } leaf z { type string; } }"#,
        r#"module m { namespace "urn:m"; prefix m; organization ACME^Corp; leaf z { type string; } }"#,
        r#"module m { namespace "urn:m"; prefix m; contact a@b^c; leaf z { type string; } }"#,
        r#"module m { namespace "urn:m"; prefix m; leaf a { type string; reference RFC:1234 ; } leaf z { type string; } }"#,
    ] {
        let tree = str_to_ast(src);
        assert_eq!(
            tree.root_node().kind(),
            "yang",
            "a symbol word in a text argument must not collapse the module: {src}"
        );
        assert!(
            !tree.root_node().has_error(),
            "text values are unquoted strings"
        );
    }
}

#[test]
fn test_quoted_text_still_parses_identically() {
    let src = r#"module m {
  namespace "urn:m";
  prefix m;
  description "normal quoted description";
  leaf a { type string; }
}"#;
    let tree = str_to_ast(src);
    assert!(!tree.root_node().has_error());
}
