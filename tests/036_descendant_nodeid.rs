//! Regression tests: `descendant-schema-nodeid` may be a bare node-identifier
//! (RFC 7950) — the optional `/…` suffix is not required, so bare
//! `unique "x"`, `unique "a b"`, and `refine x { … }` parse.

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
fn test_unique_single_leaf() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    list l {
        key "name";
        unique "start-trigger";
        leaf name { type string; }
        leaf start-trigger { type string; }
    }
}
    "#);
}

#[test]
fn test_unique_multiple_names() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    list l {
        key "name";
        unique "a b";
    }
}
    "#);
}

#[test]
fn test_refine_bare_arg() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    grouping g {
        leaf x { type string; mandatory true; }
    }
    container c {
        uses g {
            refine x { mandatory true; }
        }
    }
}
    "#);
}
