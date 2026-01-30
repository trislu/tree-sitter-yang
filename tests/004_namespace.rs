mod test_utils;

#[test]
fn test_namespace_unquoted() {
    parse_success_as!(
        r#"
module empty {
    namespace _123;
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (namespace
      arg: (uri_str))))
"#
    );
}

#[test]
fn test_namespace_quoted() {
    parse_success_as!(
        r#"
module empty {
    namespace "foo@bar.com:5070:/hello-world";
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (namespace
      arg: (uri_str))))
"#
    );
}
