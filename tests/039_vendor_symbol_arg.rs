//! Regression: unknown/vendor extension statements may carry a *bare
//! (unquoted) argument* that is not an identifier, slash-path or number —
//! RFC 7950 unquoted-string allows arbitrary non-delimiter characters (e.g.
//! `^`). Previously `m^-X` collapsed the whole module to a parse error
//! (real-world case: the IEEE 1906.1 modules in the YangModels corpus, github.com/YangModels/yang).

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
fn test_unknown_arg_with_symbols() {
    // Real-world shape: units value `m^-X` as a bare argument.
    ok(r#"
module ieee1906-dot1-information {
    yang-version 1.1;
    namespace "urn:ieee:std:1906.1:yang:ieee1906-dot1-information";
    prefix "info";
    import ieee1906-dot1-math {
        prefix ieee1906-dot1-math;
    }
    grouping information-density {
        ieee1906-dot1-math:equation information-density;
        ieee1906-dot1-si-units:units m^-X;   // vendor extension arg with '^'
        ieee1906-dot1-math:value;
    }
}
    "#);
}

#[test]
fn test_unknown_arg_symbols_with_body() {
    // Bare symbol argument followed by an opaque body.
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    ex:math ^2 {
        ex:units m^-X;
        ex:inner;
    }
}
    "#);
}

#[test]
fn test_unknown_arg_still_distinguishes_terminator_and_body() {
    // The bare-word argument must not swallow `;` or a following `{ }` body.
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    ex:units bit/s;
    ex:eq m^-X;
    ex:block m^-X {
        ex:flag;
    }
    leaf ok { type string; }
}
    "#);
}
