//! Regression: an `enum` name is a `string` in RFC 7950 (not an identifier),
//! so bare (unquoted) names may contain symbols such as `+` — e.g.
//! `enum n+1;` or `enum 2n;`. Previously any such name collapsed the whole
//! module to a parse error (real-world cases: `ietf-coms-core`, IANA registry
//! modules, MIB transcripts in the YangModels corpus). Hyphenated names such
//! as `lower-layer-down` remain ordinary identifiers.

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
fn test_enum_bare_symbol_name() {
    ok(r#"
module m {
    namespace "urn:m";
    prefix m;
    typedef redundancy {
        type enumeration {
            enum none;
            enum n+1;
            enum 2n;
        }
    }
}
    "#);
}

#[test]
fn test_hyphenated_enum_names_stay_identifiers() {
    ok(r#"
module m {
    namespace "urn:m";
    prefix m;
    typedef oper-state {
        type enumeration {
            enum lower-layer-down;
            enum not-present;
            enum dormant;
        }
    }
}
    "#);
}

#[test]
fn test_enum_quoted_and_identifier_names_still_work() {
    ok(r#"
module m {
    namespace "urn:m";
    prefix m;
    typedef e {
        type enumeration {
            enum normal;
            enum "quoted name";
        }
    }
}
    "#);
}
