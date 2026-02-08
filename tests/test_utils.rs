use tree_sitter::{Node, Tree};

#[allow(unused)]
pub fn str_to_ast(code: &str) -> Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_yang::LANGUAGE.into())
        .expect("Error loading Yang parser");
    parser.parse(code, None).unwrap()
}

#[allow(unused)]
pub fn ast_to_sexp(tree: &Tree) -> String {
    let sexp = tree_sitter::format_sexp(&tree.root_node().to_sexp(), 0);
    println!("--- The formatted s-expreesion ---\n{}", sexp);
    sexp
}

fn _iterate(node: &Node, func: fn(&Node)) {
    func(node);
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i as u32) {
            _iterate(&child, func);
        }
    }
}

#[allow(unused)]
pub fn iterate_ast(tree: &Tree, func: fn(&Node)) {
    _iterate(&tree.root_node(), func);
}

#[macro_export]
macro_rules! parse_success_as {
    ($source:expr, $expected:expr) => {
        let tree = test_utils::str_to_ast($source);
        let sexp = test_utils::ast_to_sexp(&tree);
        assert!(!tree.root_node().has_error());
        assert_eq!(sexp, $expected.to_string().trim());
    };
}

#[macro_export]
macro_rules! parse_error_as {
    ($source:expr, $expected:expr) => {
        let tree = test_utils::str_to_ast($source);
        let sexp = test_utils::ast_to_sexp(&tree);
        assert!(tree.root_node().has_error());
        assert_eq!(sexp, $expected.to_string().trim());
    };
}
