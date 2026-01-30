mod test_utils;

#[test]
fn test_empty_submodule() {
    parse_success_as!(
        r#"
submodule empty {}
"#,
        r#"
(yang
  (submodule
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
  (submodule
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
  (submodule
    arg: (identifier)
    (yang_version)
    (belongs_to
      arg: (identifier))))
"#
    );
}
