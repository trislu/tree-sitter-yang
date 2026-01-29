#[test]
fn test_line_comment() {
    let code = r#"// line comment 1
module empty {
    // line comment 2
}
// line comment 3"#;
    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_yang::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Yang parser");
    let tree = parser.parse(code, None).unwrap();
    assert!(!tree.root_node().has_error());
    let parsed_sexp = tree_sitter::format_sexp(&tree.root_node().to_sexp(), 0);
    let expected_sexp = r#"
(yang
  (comment)
  (module
    arg: (identifier)
    (comment))
  (comment))
"#;
    println!("parsed sexp=\n{}", parsed_sexp);
    assert_eq!(parsed_sexp, expected_sexp.to_string().trim());
}

#[test]
fn test_block_comment() {
    let code = r#"/*block comment 1*/ 
module /*block comment 2*/  empty /*block comment 3*/ {
    /*block comment 4*/ 
}
/*block comment 5*/ "#;
    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_yang::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Yang parser");
    let tree = parser.parse(code, None).unwrap();
    let parsed_sexp = tree_sitter::format_sexp(&tree.root_node().to_sexp(), 0);
    let expected_sexp = r#"
(yang
  (comment)
  (module
    (comment)
    arg: (identifier)
    (comment)
    (comment))
  (comment))
"#;
    println!("parsed sexp=\n{}", parsed_sexp);
    assert_eq!(parsed_sexp, expected_sexp.to_string().trim());
}
