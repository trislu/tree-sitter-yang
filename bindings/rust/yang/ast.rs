use anyhow::Context;
use easy_tree::Tree;
use tree_sitter::Parser;

use crate::{
    LANGUAGE,
    yang::{Statement, Token},
};

#[derive(Default, Clone)]
pub struct Ast {
    token_list: Vec<Token>,
    statement_tree: Tree<Statement>,
}

impl Ast {
    pub fn token_list(&self) -> &[Token] {
        &self.token_list
    }

    pub fn traverse_statements<F>(&self, mut func: F)
    where
        F: FnMut(&Statement),
    {
        let mut state = ();
        self.statement_tree
            .traverse(|_id, stmt, _state| func(stmt), |_, _, _| {}, &mut state)
    }

    pub fn parent_of(&self, stmt: &Statement) -> Option<&Statement> {
        match self.statement_tree.parent_index_unchecked(stmt.id) {
            Some(parent_id) => self.statement_tree.get(parent_id),
            None => None,
        }
    }
}

pub fn parse(source: &str) -> anyhow::Result<Ast> {
    let mut parser = Parser::new();
    let language = LANGUAGE;
    parser
        .set_language(&language.into())
        .context("Failed to set language")?;
    let tree = parser
        .parse(source, None)
        .context("Failed to parse source")?;
    let mut token_list = Vec::new();
    let mut statement_tree = Tree::new();
    let mut node_stack = vec![tree.root_node()];
    let mut stmt_stack: Vec<Option<Statement>> = vec![None];
    while let Some(node) = node_stack.pop() {
        if let Ok(token) = Token::try_from(&node) {
            token_list.push(token);
        }
        if let Ok(stmt) = Statement::try_from(&node) {
            let stmt = Statement {
                id: statement_tree.len(),
                ..stmt
            };
            if let Some(parent_stmt) = stmt_stack.last().unwrap() {
                statement_tree.add_child(parent_stmt.id, stmt.clone());
            } else {
                statement_tree.add_node(stmt.clone());
            }
            stmt_stack.push(Some(stmt));
        }
        for i in (0..node.child_count()).rev() {
            node_stack.push(node.child(i as u32).unwrap());
        }
    }
    Ok(Ast {
        token_list,
        statement_tree,
    })
}

#[cfg(test)]
mod tests {
    use crate::yang::{StatementKind, TokenKind};

    use super::*;

    #[test]
    fn test_tokennize_quoted_string() {
        let source = r#"
module test-module {
    description "bar";
}"#;
        let ast = parse(source).expect("Failed to parse source");
        let tokens = ast.token_list();

        let expected_tokens = vec![
            TokenKind::Keyword(StatementKind::Module),
            TokenKind::Argument(StatementKind::Module),
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
        let ast = parse(source).expect("Failed to parse source");
        let tokens = ast.token_list();

        let expected_tokens = vec![
            TokenKind::Keyword(StatementKind::Module),
            TokenKind::Argument(StatementKind::Module),
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
        let ast = parse(source).expect("Failed to parse source");
        let tokens = ast.token_list();
        assert!(!tokens.is_empty(), "Tokens MUST not be empty");
        // Check that some expected tokens are present and that their range text is correct
        let ranged_text_and_expected_token_vec = vec![
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
