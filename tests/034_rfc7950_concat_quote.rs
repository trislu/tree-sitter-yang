//! Regression tests for RFC 7950 §6.1.3 string handling and related parsing
//! fixes (previously these valid constructs collapsed the whole module into a
//! single ERROR node, i.e. the "full document error"):
//!
//! - '+' concatenation of quoted string arguments (path / augment / namespace)
//! - unquoted namespace URI containing ':' (no external scanner)
//! - backslash allowed inside single-quoted strings (no escape processing)
//! - `if-feature` inside an `identity` body
//! - vendor extension statements inside a `type { ... }` body

mod test_utils;

use test_utils::str_to_ast;

#[test]
fn test_path_concatenated() {
    // BBF style: a path split across several single-quoted pieces joined by '+'.
    let tree = str_to_ast(
        r#"
module test {
    namespace "urn:t";
    prefix t;
    typedef ref {
        type leafref {
            path "/t:a/t:b" + "/t:c";
        }
    }
}
    "#,
    );
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_path_concatenated_midword() {
    // Pieces need not be individually valid paths (split mid-identifier).
    let tree = str_to_ast(
        r#"
module test {
    namespace "urn:t";
    prefix t;
    typedef ref {
        type leafref {
            path '/t:a/t:forwarding-'
               + 'databases/t:b';
        }
    }
}
    "#,
    );
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_augment_concatenated() {
    let tree = str_to_ast(
        r#"
module test {
    namespace "urn:t";
    prefix t;
    augment "/t:a/t:b" + "/t:c" {
        leaf x { type string; }
    }
}
    "#,
    );
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_single_quoted_path_keeps_structure() {
    // A single quoted path must still parse as a structured schema-nodeid.
    let tree = str_to_ast(
        r#"
module test {
    namespace "urn:t";
    prefix t;
    typedef ref {
        type leafref {
            path "/t:a/t:b";
        }
    }
}
    "#,
    );
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_namespace_concatenated() {
    let tree = str_to_ast(
        r#"
module test {
    namespace "http://example.com/base/" +
      "extra";
    prefix t;
}
    "#,
    );
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_namespace_unquoted_with_colons() {
    let tree = str_to_ast(
        r#"
module test {
    namespace urn:ietf:params:xml:ns:netconf:partial-lock:1.0;
    prefix t;
}
    "#,
    );
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_single_quoted_string_with_backslash() {
    // RFC 7950: single-quoted strings perform no escape processing, so a
    // backslash is an ordinary character (common in `pattern` regexes).
    let tree = str_to_ast(
        r#"
module test {
    namespace "urn:t";
    prefix t;
    typedef ip {
        type string {
            pattern '(([^:]+:){6}(([^:]+:[^:]+)|(.*\..*)))|([0-9\.]+)';
        }
    }
}
    "#,
    );
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_identity_with_if_feature() {
    let tree = str_to_ast(
        r#"
module test {
    namespace "urn:t";
    prefix t;
    feature tls10;
    identity tls10-identity {
        if-feature "tls10";
        base t;
    }
    identity t;
}
    "#,
    );
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_type_body_with_extension() {
    // Vendor extensions are allowed inside a `type { ... }` body.
    let tree = str_to_ast(
        r#"
module test {
    namespace "urn:t";
    prefix t;
    import xs {
        prefix xs;
    }
    typedef hex {
        type xs:hexBinary {
            ex:value-length "8";
        }
    }
}
    "#,
    );
    assert!(!tree.root_node().has_error());
}
