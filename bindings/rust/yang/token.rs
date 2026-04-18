//! Token definitions for the YANG Rust binding.
//!
//! This module exposes the token kinds recognized by the YANG parser and the
//! `Token` structure used to represent a lexical token together with its source
//! range.

use std::{ops::Range, str::FromStr};

use tree_sitter::{Node, Parser};

use crate::{LANGUAGE, yang::statement::StatementKind};

/// The kind of a lexical token produced from YANG source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    /// A statement token representing a YANG statement keyword.
    Statement(StatementKind),
    /// A keyword token that appears in a statement's keyword position.
    Keyword(StatementKind),
    /// A statement argument token that appears in an argument position.
    Argument(StatementKind),
    /// A string literal token.
    StringLiteral,
    /// A numeric literal token.
    Number,
    /// A boolean literal token (`true` or `false`).
    Boolean,
    /// Treat comment as a token
    Comment,
    /// An operator token (e.g., `+`, `|`, `..`).
    Operator,
}

/// A lexical token with its kind and position in the source text.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Token {
    /// The semantic kind of the token.
    pub kind: TokenKind,
    /// The byte range of the token within the source document.
    pub range: Range<usize>,
}

/// Converts a Tree-sitter node into a `Token` when the node represents a
/// YANG token of interest.
///
/// The conversion handles statement nodes, statement argument string nodes, and
/// first-child keyword nodes inside statement nodes.
impl TryFrom<&Node<'_>> for Token {
    type Error = ();

    fn try_from(node: &Node<'_>) -> Result<Self, Self::Error> {
        let node_kind = node.kind();
        if node_kind.ends_with("_stmt") {
            let stmt_kind = StatementKind::from_str(node_kind).map_err(|_| ())?;
            return Ok(Token {
                kind: TokenKind::Statement(stmt_kind),
                range: node.byte_range(),
            });
        } else if node_kind.ends_with("_arg_str") {
            let parent = node.parent().ok_or(())?;
            let parent_kind = parent.kind();
            let stmt_kind = StatementKind::from_str(parent_kind).map_err(|_| ())?;
            return Ok(Token {
                kind: TokenKind::Argument(stmt_kind),
                range: node.byte_range(),
            });
        } else if let Some(parent) = node.parent() {
            // Check for keyword nodes that are the first child of a statement node
            if parent.kind().ends_with("_stmt")
                && parent
                    .child(0)
                    .map(|child| child.id() == node.id())
                    .unwrap_or(false)
            {
                let stmt_kind = StatementKind::from_str(parent.kind()).map_err(|_| ())?;
                return Ok(Token {
                    kind: TokenKind::Keyword(stmt_kind),
                    range: node.byte_range(),
                });
            }
            // Check for comment nodes
            if node_kind == "comment" {
                return Ok(Token {
                    kind: TokenKind::Comment,
                    range: node.byte_range(),
                });
            }
            // Check for operator nodes (this is a placeholder; adjust as needed)
            if matches!(node_kind, "+" | "|" | "..") {
                return Ok(Token {
                    kind: TokenKind::Operator,
                    range: node.byte_range(),
                });
            }
            // Check for boolean literals
            if matches!(node_kind, "true" | "false") {
                return Ok(Token {
                    kind: TokenKind::Boolean,
                    range: node.byte_range(),
                });
            }
            // Check for number literals (this is a placeholder; adjust as needed)
            if matches!(node_kind, "integer_value" | "decimal_value") {
                return Ok(Token {
                    kind: TokenKind::Number,
                    range: node.byte_range(),
                });
            }
            // Check for string literals (this is a placeholder; adjust as needed)
            if matches!(node_kind, "quoted_string") {
                return Ok(Token {
                    kind: TokenKind::StringLiteral,
                    range: node.byte_range(),
                });
            }
        }
        Err(())
    }
}

/// Errors produced while tokenizing YANG source.
#[derive(Debug)]
pub enum TokenizeError {
    /// The parser could not load the YANG language definition.
    LanguageError(String),
    /// The parser failed to build a parse tree from the source text.
    ParseError(String),
}

/// Tokenizes YANG source into a flat sequence of `Token` values.
///
/// This function initializes a Tree-sitter parser for the YANG language,
/// parses the source text, and walks the resulting syntax tree to collect
/// recognized tokens.
pub fn tokenize(source: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut parser = Parser::new();
    let language = LANGUAGE;
    if let Err(e) = parser.set_language(&language.into()) {
        return Err(TokenizeError::LanguageError(e.to_string()));
    }
    match parser.parse(source, None) {
        Some(tree) => {
            let mut tokens = Vec::new();
            let mut stack = vec![tree.root_node()];
            while let Some(node) = stack.pop() {
                //func(&node);
                if let Ok(token) = Token::try_from(&node) {
                    tokens.push(token);
                }
                for i in (0..node.child_count()).rev() {
                    stack.push(node.child(i as u32).unwrap());
                }
            }
            Ok(tokens)
        }
        None => Err(TokenizeError::ParseError(
            "Language is not loaded".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token {
            kind: TokenKind::StringLiteral,
            range: 5..15,
        };
        assert_eq!(token.kind, TokenKind::StringLiteral);
        assert_eq!(token.range, 5..15);
    }

    #[test]
    fn test_token_kinds() {
        let token = Token {
            kind: TokenKind::StringLiteral,
            range: 5..15,
        };
        assert_eq!(token.kind, TokenKind::StringLiteral);
    }

    #[test]
    fn test_token_equality() {
        let token1 = Token {
            kind: TokenKind::StringLiteral,
            range: 5..15,
        };
        let token2 = Token {
            kind: TokenKind::StringLiteral,
            range: 5..15,
        };
        assert_eq!(token1, token2);
    }

    #[test]
    fn test_token_inequality() {
        let token1 = Token {
            kind: TokenKind::StringLiteral,
            range: 5..15,
        };
        let token2 = Token {
            kind: TokenKind::Number,
            range: 5..15,
        };
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_token_hash() {
        use std::collections::HashSet;
        let token1 = Token {
            kind: TokenKind::StringLiteral,
            range: 5..15,
        };
        let token2 = Token {
            kind: TokenKind::StringLiteral,
            range: 5..15,
        };
        let mut set = HashSet::new();
        set.insert(token1.clone());
        assert!(set.contains(&token1));
        assert!(set.contains(&token2));
    }

    #[test]
    fn test_token_debug() {
        let token = Token {
            kind: TokenKind::StringLiteral,
            range: 5..15,
        };
        let debug_str = format!("{:?}", token);
        assert!(debug_str.contains("StringLiteral"));
        assert!(debug_str.contains("5..15"));
    }

    #[test]
    fn test_token_clone() {
        let token1 = Token {
            kind: TokenKind::StringLiteral,
            range: 5..15,
        };
        let token2 = token1.clone();
        assert_eq!(token1, token2);
    }

    #[test]
    fn test_token_kind_statement() {
        let token = Token {
            kind: TokenKind::Statement(StatementKind::Module),
            range: 0..6,
        };
        assert_eq!(token.kind, TokenKind::Statement(StatementKind::Module));
        assert_eq!(token.range, 0..6);
    }

    #[test]
    fn test_token_kind_keyword() {
        let token = Token {
            kind: TokenKind::Keyword(StatementKind::Module),
            range: 0..6,
        };
        assert_eq!(token.kind, TokenKind::Keyword(StatementKind::Module));
        assert_eq!(token.range, 0..6);
    }

    #[test]
    fn test_token_kind_argument() {
        let token = Token {
            kind: TokenKind::Argument(StatementKind::Module),
            range: 0..6,
        };
        assert_eq!(token.kind, TokenKind::Argument(StatementKind::Module));
        assert_eq!(token.range, 0..6);
    }

    #[test]
    fn test_token_kind_string_literal() {
        let token = Token {
            kind: TokenKind::StringLiteral,
            range: 5..15,
        };
        assert_eq!(token.kind, TokenKind::StringLiteral);
        assert_eq!(token.range, 5..15);
    }

    #[test]
    fn test_token_kind_number() {
        let token = Token {
            kind: TokenKind::Number,
            range: 5..15,
        };
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.range, 5..15);
    }

    #[test]
    fn test_token_kind_boolean() {
        let token = Token {
            kind: TokenKind::Boolean,
            range: 5..15,
        };
        assert_eq!(token.kind, TokenKind::Boolean);
        assert_eq!(token.range, 5..15);
    }

    #[test]
    fn test_tokennize_quoted_string() {
        let source = r#"
module test-module {
    description "bar";
}"#;
        let tokens = tokenize(source).expect("Failed to tokenize source");

        let expected_tokens = vec![
            TokenKind::Statement(StatementKind::Module),
            TokenKind::Keyword(StatementKind::Module),
            TokenKind::Argument(StatementKind::Module),
            TokenKind::Statement(StatementKind::Description),
            TokenKind::Keyword(StatementKind::Description),
            TokenKind::Argument(StatementKind::Description),
            TokenKind::StringLiteral,
        ];
        // Check that the tokens contain the expected kinds in the expected order
        let mut token_kinds = tokens.iter().map(|token| token.kind.clone());
        for expected_kind in expected_tokens {
            let token_kind = token_kinds.next().unwrap();
            assert_eq!(token_kind, expected_kind);
        }
    }

    #[test]
    fn test_tokennize_concatenated_string() {
        let source = r#"
module test-module {
    description "foo" + "bar";
}"#;
        let tokens = tokenize(source).expect("Failed to tokenize source");

        let expected_tokens = vec![
            TokenKind::Statement(StatementKind::Module),
            TokenKind::Keyword(StatementKind::Module),
            TokenKind::Argument(StatementKind::Module),
            TokenKind::Statement(StatementKind::Description),
            TokenKind::Keyword(StatementKind::Description),
            TokenKind::Argument(StatementKind::Description),
            TokenKind::StringLiteral,
            TokenKind::Operator,
            TokenKind::StringLiteral,
        ];
        // Check that the tokens contain the expected kinds in the expected order
        let mut token_kinds = tokens.iter().map(|token| token.kind.clone());
        for expected_kind in expected_tokens {
            let token_kind = token_kinds.next().unwrap();
            assert_eq!(token_kind, expected_kind);
        }
    }

    #[test]
    fn test_tokenize() {
        let source = r#"
module test-module {
    namespace "http://example.com/test-module";
    prefix tm;
    container test-container {
        leaf test-leaf {
            type string;
        }
    }
}"#;
        let tokens = tokenize(source).expect("Failed to tokenize source");
        assert!(!tokens.is_empty(), "Tokens MUST not be empty");
        // Check that some expected tokens are present and that their range text is correct
        let ranged_text_and_expected_token_vec = vec![
            (
                r#"module test-module {
    namespace "http://example.com/test-module";
    prefix tm;
    container test-container {
        leaf test-leaf {
            type string;
        }
    }
}"#,
                Token {
                    kind: TokenKind::Statement(StatementKind::Module),
                    range: 1..183,
                },
            ),
            (
                r#"module"#,
                Token {
                    kind: TokenKind::Keyword(StatementKind::Module),
                    range: 1..7,
                },
            ),
            (
                r#"test-module"#,
                Token {
                    kind: TokenKind::Argument(StatementKind::Module),
                    range: 8..19,
                },
            ),
            (
                r#"namespace "http://example.com/test-module";"#,
                Token {
                    kind: TokenKind::Statement(StatementKind::Namespace),
                    range: 26..69,
                },
            ),
            (
                r#"namespace"#,
                Token {
                    kind: TokenKind::Keyword(StatementKind::Namespace),
                    range: 26..35,
                },
            ),
            (
                r#""http://example.com/test-module""#,
                Token {
                    kind: TokenKind::Argument(StatementKind::Namespace),
                    range: 36..68,
                },
            ),
            (
                r#"prefix tm;"#,
                Token {
                    kind: TokenKind::Statement(StatementKind::Prefix),
                    range: 74..84,
                },
            ),
            (
                r#"prefix"#,
                Token {
                    kind: TokenKind::Keyword(StatementKind::Prefix),
                    range: 74..80,
                },
            ),
            (
                r#"tm"#,
                Token {
                    kind: TokenKind::Argument(StatementKind::Prefix),
                    range: 81..83,
                },
            ),
            (
                r#"container test-container {
        leaf test-leaf {
            type string;
        }
    }
"#,
                Token {
                    kind: TokenKind::Statement(StatementKind::Container),
                    range: 89..182,
                },
            ),
            (
                r#"container"#,
                Token {
                    kind: TokenKind::Keyword(StatementKind::Container),
                    range: 89..98,
                },
            ),
            (
                r#"test-container"#,
                Token {
                    kind: TokenKind::Argument(StatementKind::Container),
                    range: 99..113,
                },
            ),
            (
                r#"leaf test-leaf {
            type string;
        }"#,
                Token {
                    kind: TokenKind::Statement(StatementKind::Leaf),
                    range: 124..175,
                },
            ),
            (
                r#"leaf"#,
                Token {
                    kind: TokenKind::Keyword(StatementKind::Leaf),
                    range: 124..128,
                },
            ),
            (
                r#"test-leaf"#,
                Token {
                    kind: TokenKind::Argument(StatementKind::Leaf),
                    range: 129..138,
                },
            ),
            (
                r#"type string;"#,
                Token {
                    kind: TokenKind::Statement(StatementKind::Type),
                    range: 153..165,
                },
            ),
            (
                r#"type"#,
                Token {
                    kind: TokenKind::Keyword(StatementKind::Type),
                    range: 153..157,
                },
            ),
            (
                r#"string"#,
                Token {
                    kind: TokenKind::Argument(StatementKind::Type),
                    range: 158..164,
                },
            ),
        ];
        for (expected_text, expected_token) in ranged_text_and_expected_token_vec {
            assert!(
                tokens.iter().any(|token| token == &expected_token),
                "Expected token {:?} not found",
                expected_token
            );
            assert_eq!(
                &source[expected_token.range.clone()],
                expected_text,
                "Expected range text for token {:?}",
                expected_token
            );
        }
    }
}
