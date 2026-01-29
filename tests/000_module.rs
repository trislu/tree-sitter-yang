#[test]
fn test_empty_module() {
    let code = r#"module empty {}"#;
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
  (module
    arg: (identifier)))
"#;
    println!("parsed sexp=\n{}", parsed_sexp);
    assert_eq!(parsed_sexp, expected_sexp.to_string().trim());
}

#[test]
fn test_module() {
    let code = r#"
    module foo-me-once {
        yang-version 1.1;
        prefix foo;
        namespace "urn:example:foo-me-once";
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
  (module
    arg: (identifier)
    (yang_version)
    (prefix
      arg: (identifier))
    (namespace
      arg: (string))))
"#;
    println!("parsed sexp=\n{}", parsed_sexp);
    assert_eq!(parsed_sexp, expected_sexp.to_string().trim());
}
