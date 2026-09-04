//! Regression tests: bare (unquoted) string arguments that are not valid
//! identifiers or numbers — digit-starting `enum` names, slash-containing
//! `units`/`default` values, and numeric unknown-statement arguments.

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
fn test_enum_digit_starting_name() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    typedef speed {
        type enumeration {
            enum 10g-base-er {
                description "10G Base-ER.";
            }
            enum 5-seconds;
        }
    }
}
    "#);
}

#[test]
fn test_units_bare_slash() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    leaf speed { type uint64; units Mb/s; }
}
    "#);
}

#[test]
fn test_default_bare_slash() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    leaf url {
        type string;
        default scheme://host/path;
    }
}
    "#);
}

#[test]
fn test_unknown_numeric_arg() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    container c {
        ex:sort-priority 1;
    }
}
    "#);
}
