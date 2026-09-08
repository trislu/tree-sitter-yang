//! Regression: a vendor extension statement (unknown-stmt) interspersed among
//! `pattern`/`length` inside `type string { ... }` must not collapse the
//! whole module (real-world trigger: OpenConfig's `openconfig-yang-types.yang`,
//! whose `hex-string-prefixed` typedef writes `pattern`, then
//! `oc-ext:posix-pattern`, then `length`, in that order). Two statements of
//! that shape parsed fine; a third - specifically `length` following an
//! extension statement - didn't, because `_string_restrictions` only allowed
//! an unknown-stmt before/after the whole pattern/length group, not between
//! its members. Fixed by letting `_string_restrictions` accept any order/count
//! of length/pattern/unknown-stmt (RFC 7950 allows an unknown-statement as a
//! substatement of anything; a semantic validator, not this grammar, is the
//! right place to reject e.g. two length-stmts).

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
fn extension_between_pattern_and_length_parses() {
    ok(r#"
module test {
    namespace "urn:test";
    prefix t;
    import openconfig-extensions { prefix "oc-ext"; }

    typedef hex-string-prefixed {
        type string {
            pattern '(0x)([0-9a-fA-F]{2})*';
            oc-ext:posix-pattern '^(0x)([0-9a-fA-F]{2})*$';
            length "3..max";
        }
        description "A string encoding a hexadecimal number with a prefix of '0x'.";
    }
}
    "#);
}

/// Two patterns each paired with a trailing extension, then length.
#[test]
fn extension_after_each_of_several_patterns_then_length_parses() {
    ok(r#"
module test {
    namespace "urn:test";
    prefix t;
    import openconfig-extensions { prefix "oc-ext"; }

    typedef x {
        type string {
            pattern 'a';
            oc-ext:posix-pattern '^a$';
            pattern 'b';
            oc-ext:posix-pattern '^b$';
            length "1..2";
        }
    }
}
    "#);
}

/// The plain (no extensions) forms from RFC 7950's grammar still parse.
#[test]
fn plain_forms_still_parse() {
    ok(r#"
module test {
    namespace "urn:test";
    prefix t;
    typedef a { type string { length "0..64"; pattern "x"; } }
    typedef b { type string { pattern "x"; pattern "y"; length "0..64"; } }
    typedef c { type string { pattern "x"; } }
}
    "#);
}
