//! Regression: `unique "…" + "…"` and `key "…" + "…"` (RFC 7950 string
//! concatenation in the argument) must parse — previously the trailing
//! `+ quoted-string` was unexpected, and recovery collapsed the whole module
//! (one real-world trigger: a draft TE module whose `list lsp` writes
//! `unique "a b c d " + "e";`; a draft BGP-LS module writes a concatenated
//! `key`). pyang accepts both forms.

mod test_utils;

use test_utils::str_to_ast;

fn ok(src: &str) {
    let tree = str_to_ast(src);
    assert!(
        !tree.root_node().has_error(),
        "expected no parse error in:\n{src}"
    );
}

#[test]
fn unique_concat_parses() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    list l {
        key "name";
        unique "a b c d "
          + "e";
        leaf name { type string; }
        leaf a { type string; }
        leaf b { type string; }
        leaf c { type string; }
        leaf d { type string; }
        leaf e { type string; }
    }
}
    "#);
}

#[test]
fn key_concat_parses() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    list l {
        key "a "
          + "b";
        leaf a { type string; }
        leaf b { type string; }
    }
}
    "#);
}

/// The plain (non-concatenated) forms keep parsing with the structured
/// argument, unchanged.
#[test]
fn plain_forms_still_parse() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    list l {
        key "ip port";
        unique "domain/user";
        leaf ip { type string; }
        leaf port { type string; }
        container domain { leaf user { type string; } }
    }
}
    "#);
}
