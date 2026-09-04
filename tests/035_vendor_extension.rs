//! Regression tests: robust handling of vendor extension statements.
//!
//! - an unknown-statement with no argument but a `{ ... }` body and whitespace
//!   before the brace (previously the optional-argument separator ate the
//!   whitespace and the parser could not backtrack);
//! - an unknown-statement whose `{ ... }` body contains nested unknown
//!   statements or even YANG-looking content (bodies are opaque, so the braces
//!   can no longer be confused with the enclosing statement body);
//! - vendor `extension`-definition bodies (argument + nested extension
//!   statements);
//! - `uses 'mod:grouping' + 'suffix'` concatenated quoted arguments.
//!
//! A neutral `ex:` prefix stands in for any vendor extension prefix.

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
fn test_unknown_no_arg_with_body() {
    // Unknown statement with no argument but a block, whitespace before '{'.
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    container a {
        ex:validate {
            ex:internal;
        }
    }
}
    "#);
}

#[test]
fn test_unknown_body_with_yang_content() {
    // Unknown body content that *looks* like YANG (type string;) must stay opaque.
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    container a {
        ex:arg-type {
            type string;
        }
    }
}
    "#);
}

#[test]
fn test_unknown_nested_blocks() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    container a {
        ex:validate "cb" {
            ex:internal;
            ex:dependency ".";
        }
    }
}
    "#);
}

#[test]
fn test_extension_definition_with_extension_substatements() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    extension use-in {
        argument name {
            ex:arg-type {
                type string;
            }
        }
        ex:use-in "extension";
        ex:substatement "type" {
            ex:occurence "1";
        }
    }
}
    "#);
}

#[test]
fn test_uses_concatenated_grouping_ref() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    grouping a-very-long-grouping-name-as-argument-string {
        leaf a { type string; }
    }
    container c {
        uses 'mod:a-very-long-grouping-name' + '-as-argument-string';
    }
}
    "#);
}

#[test]
fn test_unknown_with_quoted_arg_and_body() {
    ok(r#"
module test {
    namespace "urn:t";
    prefix t;
    container a {
        ex:callpoint "cp" {
            ex:transform "x";
        }
    }
}
    "#);
}
