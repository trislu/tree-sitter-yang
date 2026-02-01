mod test_utils;

#[test]
fn test_empty_submodule() {
    parse_success_as!(
        r#"
submodule test {}
"#,
        r#"
(yang
  (submodule_stmt
    arg: (identifier)))
"#
    );
}

#[test]
fn test_submodule_name() {
    parse_success_as!(
        r#"
submodule "name-can-be-string" {}
"#,
        r#"
(yang
  (submodule_stmt
    arg: (identifier)))
"#
    );
}

#[test]
fn test_submodule() {
    parse_success_as!(
        r#"
submodule "sub-me-do" {
    yang-version 1.1;
    belongs-to me-too;
}
    "#,
        r#"
(yang
  (submodule_stmt
    arg: (identifier)
    (yang_version_stmt)
    (belongs_to_stmt
      arg: (identifier))))
"#
    );
}
