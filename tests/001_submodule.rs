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
    arg: (submodule_arg_str
      (identifier))))
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
    arg: (submodule_arg_str
      (identifier))))
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
    arg: (submodule_arg_str
      (identifier))
    (yang_version_stmt
      arg: (yang_version_arg_str))
    (belongs_to_stmt
      arg: (belongs_to_arg_str
        (identifier)))))
"#
    );
}
