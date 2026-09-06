//! The shipped `queries/highlights.scm` must stay a valid, compiling query
//! against this grammar and must keep producing its headline captures on a
//! representative module. Guards against a grammar edit silently making a
//! capture pattern impossible (tree-sitter rejects such patterns) or renaming
//! a node out from under the highlight set.

mod test_utils;

use tree_sitter::{Query, StreamingIterator};

const HIGHLIGHTS: &str = include_str!("../queries/highlights.scm");

fn sample() -> &'static str {
    r#"// leading comment
module app {
    yang-version 1.1;
    namespace "urn:app";
    prefix app;
    /* block comment */
    description "A demo module.";
    revision 2024-01-01 { description "first"; }
    import ietf-inet-types { prefix inet; revision-date 2013-07-15; }
    include app-sub;
    typedef t { type inet:ip-address; }
    typedef e { type enumeration { enum n+1; enum "two words"; } }
    identity id { base inet:ip-version; }
    feature f { if-feature "app:g"; }
    feature g;
    grouping grp { leaf gl { type string; } }
    container c {
        config false;
        leaf l { type t; default "x"; }
        leaf on { type boolean; default true; }
        leaf num { type uint8; default 7; }
        leaf-list ll { type string; min-elements 1; max-elements 8; }
        list ls { key "a b"; unique "a b"; leaf a { type string; } leaf b { type string; } }
        choice ch { case ca { leaf x { type string; } } }
        uses app:grp;
        leaf r { type leafref { path "/c/ls/a"; } }
        leaf dt { type string; units meter^2.second-1; }
    }
    rpc op { input { leaf i { type string; } } output { leaf o { type string; } } }
    notification alarm { leaf nn { type string; } }
    augment "/c" { leaf z { type string; } }
    deviation "/c/l" { deviate not-supported; }
}"#
}

#[test]
fn highlights_query_compiles() {
    Query::new(&tree_sitter_yang::LANGUAGE.into(), HIGHLIGHTS)
        .expect("queries/highlights.scm must stay a valid query for the grammar");
}

fn count_captures(source: &str, want: &str) -> usize {
    let query = Query::new(&tree_sitter_yang::LANGUAGE.into(), HIGHLIGHTS).unwrap();
    let tree = test_utils::str_to_ast(source);
    let mut cursor = tree_sitter::QueryCursor::new();
    let mut count = 0usize;
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    while let Some(m) = matches.next() {
        for c in m.captures {
            if query.capture_names()[c.index as usize] == want {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn highlights_cover_the_headline_classes() {
    let src = sample();
    let ast = test_utils::str_to_ast(src);
    assert!(!ast.root_node().has_error(), "sample must parse clean");
    // Each class must fire on the sample (loose lower bounds; the query set is
    // tuned for coverage, not exact counts).
    for want in [
        "keyword",
        "comment",
        "string",
        "namespace",
        "type",
        "variable",
        "boolean",
    ] {
        assert!(
            count_captures(src, want) >= 1,
            "capture class '{want}' produced no match on the highlight sample"
        );
    }
}
