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
    (submodule_keyword)
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
    (submodule_keyword)
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
    belongs-to me-too {
        prefix mt;
    }
}
    "#,
        r#"
(yang
  (submodule_stmt
    (submodule_keyword)
    arg: (submodule_arg_str
      (identifier))
    (yang_version_stmt
      (yang_version_keyword)
      arg: (yang_version_arg_str))
    (belongs_to_stmt
      (belongs_to_keyword)
      arg: (belongs_to_arg_str
        (identifier))
      (prefix_stmt
        (prefix_keyword)
        arg: (prefix_arg_str
          (identifier))))))
"#
    );
}
