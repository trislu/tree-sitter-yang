//! Token definitions for the YANG Rust binding.
//!
//! This module exposes the token kinds recognized by the YANG parser and the
//! `Token` structure used to represent a lexical token together with its source
//! range.

use std::{ops::Range, str::FromStr};

use tree_sitter::Node;

use crate::yang::StatementKind;

/// The kind of a lexical token produced from YANG source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
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
        if node_kind.ends_with("_arg_str") {
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
}
