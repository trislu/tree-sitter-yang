mod test_utils;

#[test]
fn test_namespace_unquoted() {
    parse_success_as!(
        r#"
module test {
    namespace _123;
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (module_arg_str
      (identifier))
    (namespace_stmt
      arg: (namespace_arg_str))))
"#
    );
}

#[test]
fn test_namespace_quoted() {
    parse_success_as!(
        r#"
module test {
    namespace "foo@bar.com:5070:/hello-world";
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (module_arg_str
      (identifier))
    (namespace_stmt
      arg: (namespace_arg_str))))
"#
    );
}
