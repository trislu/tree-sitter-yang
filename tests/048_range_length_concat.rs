//! Regression: `range "…" + "…"` and `length "…" + "…"` (RFC 7950 string
//! concatenation inside the argument) must parse — previously the trailing
//! `+ quoted-string` was unexpected and recovery collapsed the whole module
//! (real-world trigger: an extracted draft types module writing
//! `range "16 | 17 | 32 | 33 | …" + " 80 | …";`). pyang accepts both forms.

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
fn range_concat_parses() {
    ok(r#"
module test {
    yang-version 1.1;
    prefix t;
    namespace "urn:t";
    typedef component-type-id {
        type uint8 {
            range "16 | 17 | 32 | 33 | 35 | 48 | 64 | 65 |"
            + " 80 | 81 | 96 | 112 | 128";
        }
    }
}
    "#);
}

#[test]
fn length_concat_parses() {
    ok(r#"
module test {
    yang-version 1.1;
    prefix t;
    namespace "urn:t";
    typedef s {
        type string {
            length "1 | 2 | 3 | 4 |"
            + " 8 | 16";
        }
    }
}
    "#);
}

/// The plain (non-concatenated) forms still parse.
#[test]
fn plain_forms_still_parse() {
    ok(r#"
module test {
    yang-version 1.1;
    prefix t;
    namespace "urn:t";
    typedef a { type int32 { range "1..4 | 10..20"; } }
    typedef b { type string { length "0..64"; } }
}
    "#);
}
