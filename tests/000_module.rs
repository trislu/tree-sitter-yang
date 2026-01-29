mod test_utils;

#[test]
fn test_empty_module() {
    parse_success_as!(
        r#"
module empty {}
"#,
        r#"
(yang
  (module
    arg: (identifier)))
"#
    );
}

#[test]
fn test_module() {
    parse_success_as!(
        r#"
module foo-me-once {
    yang-version 1.1;
    prefix foo;
    namespace "urn:example:foo-me-once";
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (yang_version)
    (prefix
      arg: (identifier))
    (namespace
      arg: (string))))
"#
    );
}
