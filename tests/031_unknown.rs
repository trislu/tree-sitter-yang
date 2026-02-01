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
    arg: (identifier)
    (unknown_stmt
      (prefix
        (identifier))
      (identifier)
      (string))))
        "#
    );
}
