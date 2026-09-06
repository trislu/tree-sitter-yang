//! Regression: the standard `units` statement accepts a *bare (unquoted)
//! string* argument (RFC 7950 unquoted-string), which may contain symbols
//! such as `^` — e.g. `units meter^2.second-1;`. Previously any such value
//! collapsed the whole module to a parse error (real-world case: the IEEE 1906.1 units modules in the YangModels corpus, github.com/YangModels/yang).

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
fn test_units_bare_symbol_arg() {
    ok(r#"
module m {
    namespace "urn:m";
    prefix m;
    typedef diffusion-coefficient {
        type uint32;
        units meter^2.second-1;
    }
    leaf a {
        type uint32;
        units kilogram.second^-1.meter^-2;
    }
}
    "#);
}

#[test]
fn test_units_still_accepts_quoted_and_slash_args() {
    ok(r#"
module m {
    namespace "urn:m";
    prefix m;
    typedef q { type uint32; units "m^-X"; }
    typedef s { type uint32; units meter/second; }
    typedef i { type uint32; units meter^2.second-1; }
}
    "#);
}
