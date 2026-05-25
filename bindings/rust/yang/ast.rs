use easy_tree::Tree;
use tree_sitter::Parser;

use crate::{
    LANGUAGE,
    yang::{Statement, StatementKind, Token},
};

#[derive(Default, Clone)]
pub struct Ast {
    token_list: Vec<Token>,
    statement_tree: Tree<Statement>,
}

impl Ast {
    pub fn parse(source: &str) -> Option<Ast> {
        // make sure the source is valid UTF-8 before parsing, since Tree-sitter requires valid UTF-8 input
        let _ = str::from_utf8(source.as_bytes()).ok()?;
        let mut parser = Parser::new();
        let language = LANGUAGE;
        // shall not failed since the language is statically linked and valid
        parser.set_language(&language.into()).ok()?;
        // shall not failed since the language is valid and the source is valid UTF-8
        // TODO: parse with old_tree to enable incremental parsing and better performance for large documents
        let tree = parser.parse(source, None)?;
        let mut token_list = Vec::new();
        let mut statement_tree = Tree::new();
        let mut parse_stack = vec![(tree.root_node(), None)];
        while let Some((node, parent)) = parse_stack.pop() {
            if let Ok(token) = Token::try_from(&node) {
                token_list.push(token);
            }
            let mut parent = parent;
            if let Ok(stmt) = Statement::try_from(&node) {
                // updating the statement id after adding it to the tree is kind of hacky,
                // but the id is assigned by the tree and not known beforehand
                if let Some(parent_id) = parent {
                    let child_id = statement_tree.add_child(parent_id, stmt.clone());
                    statement_tree.get_unchecked_mut(child_id).id = child_id;
                    parent = Some(child_id);
                } else {
                    let root_id = statement_tree.add_node(stmt.clone());
                    statement_tree.get_unchecked_mut(root_id).id = root_id;
                    parent = Some(root_id);
                }
            }
            for i in (0..node.child_count()).rev() {
                parse_stack.push((node.child(i as u32).unwrap(), parent));
            }
        }
        Some(Ast {
            token_list,
            statement_tree,
        })
    }

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

    pub fn search_children(&self, stmt: &Statement, kind: StatementKind) -> Vec<&Statement> {
        let mut children = vec![];
        if let Some(stmt) = self.statement_tree.get(stmt.id) {
            for child_id in self.statement_tree.children(stmt.id) {
                let child_stmt = self.statement_tree.get_unchecked(*child_id);
                if child_stmt.kind == kind {
                    children.push(child_stmt);
                }
            }
        }
        children
    }
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
        let ast = Ast::parse(source).expect("Failed to parse source");
        let tokens = ast.token_list();

        let expected_tokens = [
            TokenKind::Keyword(StatementKind::Module),
            TokenKind::Argument(StatementKind::Module),
            TokenKind::Keyword(StatementKind::Description),
            TokenKind::Argument(StatementKind::Description),
            TokenKind::StringLiteral,
        ];
        // Check that the tokens contain the expected kinds in the expected order
        for (id, expected_token) in expected_tokens.iter().enumerate() {
            assert_eq!(&tokens[id].kind, expected_token);
        }
    }

    #[test]
    fn test_tokennize_concatenated_string() {
        let source = r#"
module test-module {
    description "foo" + "bar";
}"#;
        let ast = Ast::parse(source).expect("Failed to parse source");
        let tokens = ast.token_list();

        let expected_tokens = [
            TokenKind::Keyword(StatementKind::Module),
            TokenKind::Argument(StatementKind::Module),
            TokenKind::Keyword(StatementKind::Description),
            TokenKind::Argument(StatementKind::Description),
            TokenKind::StringLiteral,
            TokenKind::Operator,
            TokenKind::StringLiteral,
        ];
        // Check that the tokens contain the expected kinds in the expected order
        for (id, expected_token) in expected_tokens.iter().enumerate() {
            assert_eq!(&tokens[id].kind, expected_token);
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
        let ast = Ast::parse(source).expect("Failed to parse source");
        let tokens = ast.token_list();
        assert!(!tokens.is_empty(), "Tokens MUST not be empty");
        // Check that some expected tokens are present and that their range text is correct
        let expected = [
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
        for (id, (expected_text, expected_token)) in expected.iter().enumerate() {
            let parsed_token = &tokens[id];
            assert_eq!(expected_token, parsed_token);
            assert_eq!(
                &source[parsed_token.range.clone()],
                *expected_text,
                "Expected range text for token {:?}",
                parsed_token
            );
        }
    }

    #[test]
    fn test_traverse_statement() {
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
        let ast = Ast::parse(source).expect("Failed to parse source");
        let mut statements = vec![];
        ast.traverse_statements(|stmt| {
            statements.push(stmt.clone());
        });

        let expected_statements = [
            Statement {
                id: 0,
                kind: StatementKind::Module,
                keyword: 1..7,
                argument: Some(8..19),
            },
            Statement {
                id: 1,
                kind: StatementKind::Namespace,
                keyword: 26..35,
                argument: Some(36..68),
            },
            Statement {
                id: 2,
                kind: StatementKind::Prefix,
                keyword: 74..80,
                argument: Some(81..83),
            },
            Statement {
                id: 3,
                kind: StatementKind::Container,
                keyword: 89..98,
                argument: Some(99..113),
            },
            Statement {
                id: 4,
                kind: StatementKind::Leaf,
                keyword: 124..128,
                argument: Some(129..138),
            },
            Statement {
                id: 5,
                kind: StatementKind::Type,
                keyword: 153..157,
                argument: Some(158..164),
            },
        ];

        for (id, expected_stmt) in expected_statements.iter().enumerate() {
            assert_eq!(expected_stmt, &statements[id]);
        }
    }

    #[test]
    fn test_parent_of() {
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
        let ast = Ast::parse(source).expect("Failed to parse source");
        let mut statements = vec![];
        ast.traverse_statements(|stmt| {
            statements.push(stmt.clone());
        });

        let expected_parents = [
            None,
            Some(Statement {
                // the parent of the namespace statement is the module statement
                id: 0,
                kind: StatementKind::Module,
                keyword: 1..7,
                argument: Some(8..19),
            }),
            Some(Statement {
                // the parent of the prefix statement is the module statement
                id: 0,
                kind: StatementKind::Module,
                keyword: 1..7,
                argument: Some(8..19),
            }),
            Some(Statement {
                // the parent of the container statement is the module statement
                id: 0,
                kind: StatementKind::Module,
                keyword: 1..7,
                argument: Some(8..19),
            }),
            Some(Statement {
                // the parent of the leaf statement is the container statement
                id: 3,
                kind: StatementKind::Container,
                keyword: 89..98,
                argument: Some(99..113),
            }),
            Some(Statement {
                // the parent of the type statement is the leaf statement
                id: 4,
                kind: StatementKind::Leaf,
                keyword: 124..128,
                argument: Some(129..138),
            }),
        ];

        for (id, expected_parent) in expected_parents.iter().enumerate() {
            let stmt = &statements[id];
            assert_eq!(
                expected_parent.clone(),
                ast.parent_of(stmt).cloned(),
                "Expected parent {:?} for statement with id {}",
                expected_parent,
                stmt.id
            );
        }
    }

    #[test]
    fn test_search_children() {
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
        let ast = Ast::parse(source).expect("Failed to parse source");
        ast.traverse_statements(|stmt| match stmt.kind {
            StatementKind::Module => {
                let namespace = ast.search_children(stmt, StatementKind::Namespace);
                assert_eq!(namespace.len(), 1);
                assert_eq!(namespace[0].kind, StatementKind::Namespace);
                let prefix = ast.search_children(stmt, StatementKind::Prefix);
                assert_eq!(prefix.len(), 1);
                assert_eq!(prefix[0].kind, StatementKind::Prefix);
            }
            StatementKind::Container => {
                let children = ast.search_children(stmt, StatementKind::Leaf);
                assert_eq!(children.len(), 1);
                assert_eq!(children[0].kind, StatementKind::Leaf);
            }
            StatementKind::Leaf => {
                let children = ast.search_children(stmt, StatementKind::Type);
                assert_eq!(children.len(), 1);
                assert_eq!(children[0].kind, StatementKind::Type);
            }
            _ => {}
        });
    }
}
