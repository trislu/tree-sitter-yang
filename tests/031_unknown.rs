mod test_utils;

#[test]
fn test_unknown() {
    parse_success_as!(
        r#"
module test{
    x:y "/foo/bar";
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (module_arg_str
      (identifier))
    (unknown_stmt
      (prefix
        (identifier))
      (identifier)
      (string))))
        "#
    );
}
