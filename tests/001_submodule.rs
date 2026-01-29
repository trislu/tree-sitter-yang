#[test]
fn test_empty_submodule() {
    let code = r#"submodule empty {}"#;
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
  (submodule
    arg: (identifier)))
"#;
    assert_eq!(parsed_sexp, expected_sexp.to_string().trim());
}

#[test]
fn test_submodule() {
    let code = r#"
    submodule "sub-me-do" {
        yang-version 1.1;
        belongs-to me-too;
    }
    "#;
    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_yang::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Yang parser");
    let tree = parser.parse(code, None).unwrap();
    let parsed_sexp = tree_sitter::format_sexp(&tree.root_node().to_sexp(), 0);
    let expected_sexp = r#"
(yang
  (submodule
    arg: (identifier)
    (yang_version)
    (belongs_to
      arg: (identifier))))
"#;
    println!("parsed sexp=\n{}", parsed_sexp);
    assert_eq!(parsed_sexp, expected_sexp.to_string().trim());
}
