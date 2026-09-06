//! Regression: double-quoted strings in real modules carry backslash escapes
//! beyond RFC 7950's `\n \t \" \\` — e.g. `\*`, `\S`, `\.` inside `pattern`.
//! pyang tolerates these (it emits only a warning). Previously the grammar
//! rejected them and the whole module collapsed (ietf-netconf-acm,
//! ietf-ipfix-psamp, DRAFT ietf-isis).

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
fn test_pattern_escapes_in_double_quotes() {
    ok(r#"
module m {
    namespace "urn:m";
    prefix m;
    typedef matchall-string-type {
        type string {
            pattern "\*";
        }
    }
    typedef name-type {
        type string {
            pattern "\S(.*\S)?";
        }
    }
    typedef lsp-id {
        type string {
            pattern "[0-9A-Fa-f]{4}\.[0-9A-Fa-f]{4}\.[0-9A-Fa-f]{4}\.[0-9A-Fa-f]{4}";
        }
    }
}
    "#);
}

#[test]
fn test_pattern_concatenation_across_lines() {
    // `+` string concatenation may span lines; each piece may hold escapes.
    ok(r#"
module m {
    namespace "urn:m";
    prefix m;
    typedef lsp-id {
        type string {
            pattern "[0-9A-Fa-f]{4}\.[0-9A-Fa-f]{4}\."
                 + "[0-9A-Fa-f]{4}\.[0-9A-Fa-f]{4}";
        }
    }
}
    "#);
}

#[test]
fn test_standard_escapes_still_work() {
    ok(r#"
module m {
    namespace "urn:m";
    prefix m;
    leaf a {
        type string {
            pattern "\n\t\"\\";
        }
        description "a \"quoted\" word";
    }
}
    "#);
}
