//! This crate provides [`YANG`] language support for the [tree-sitter] parsing library.
//!
//! Typically, you will use the [`LANGUAGE`] constant to add this language to a
//! tree-sitter [`Parser`], and then use the parser to parse some code:
//!
//! ```
//! let code = r#"
//! // module tree-sitter
//! module tree-sitter {
//!     yang-version 1.1;
//!     prefix ts;
//!     namespace "https://tree-sitter.github.io/tree-sitter/";
//!     revision 2026-01-31;
//!     import tree-sitter-yang {
//!         prefix ts-yang;
//!         revision-date 2026-01-31;
//!         description
//!         "Please see below YANG RFCs:
//!         https://www.rfc-editor.org/rfc/rfc6020
//!         https://www.rfc-editor.org/rfc/rfc7950 (version 1.1)
//!         https://www.rfc-editor.org/rfc/rfc9890";
//!         reference "https://github.com/trislu/tree-sitter-yang";
//!     }
//!
//!     extension parser {
//!         description
//!         "The YANG language parser extension";
//!         argument "parse" {
//!             yin-element "true";
//!         }
//!         reference 'https://tree-sitter.github.io/tree-sitter/using-parsers/index.html';
//!         status current;
//!     }
//! }
//! "#;
//! let mut parser = tree_sitter::Parser::new();
//! let language = tree_sitter_yang::LANGUAGE;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading Yang parser");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! let sexp = tree_sitter::format_sexp(&tree.root_node().to_sexp(), 0);
//! println!("the s-expression:\n{}", sexp);
//! ```
//!
//! [`Parser`]: https://docs.rs/tree-sitter/0.26.3/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/
//! [`YANG`]: https://www.rfc-editor.org/rfc/rfc6020

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_yang() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_yang) };

/// The content of the [`node-types.json`] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

#[cfg(with_highlights_query)]
/// The syntax highlighting query for this grammar.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");

#[cfg(with_injections_query)]
/// The language injection query for this grammar.
pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");

#[cfg(with_locals_query)]
/// The local variable query for this grammar.
pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");

#[cfg(with_tags_query)]
/// The symbol tagging query for this grammar.
pub const TAGS_QUERY: &str = include_str!("../../queries/tags.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading YANG parser");
    }
}
