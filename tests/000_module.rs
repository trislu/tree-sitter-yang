mod test_utils;

#[test]
fn test_empty_module() {
    parse_success_as!(
        r#"
module test {}
"#,
        r#"
(yang
  (module_stmt
    arg: (identifier)))
"#
    );
}

#[test]
fn test_module_name() {
    parse_success_as!(
        r#"
module "name-can-be-string" {}
"#,
        r#"
(yang
  (module_stmt
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
  (module_stmt
    arg: (identifier)
    (yang_version)
    (prefix_stmt
      arg: (identifier))
    (namespace_stmt
      arg: (uri_str))))
"#
    );
}
