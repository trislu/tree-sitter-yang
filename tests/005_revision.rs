mod test_utils;

#[test]
fn test_single_revision() {
    parse_success_as!(
        r#"
module rev-test {
    revision 2026-01-30 {}
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (revision
      arg: (revision_date_str))))
"#
    );
}

#[test]
fn test_multiple_revisions() {
    parse_success_as!(
        r#"
module rev-test {
    revision 2026-01-30 {}
    revision "2026-01-31" {}
    revision '2026-02-01' {}
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (revision
      arg: (revision_date_str))
    (revision
      arg: (revision_date_str))
    (revision
      arg: (revision_date_str))))
"#
    );
}

#[test]
fn test_bad_revisions() {
    parse_error_as!(
        r#"
module rev-test {
    revision 2026-1-31 {}
    revision 2026-02-1 {}
    revision 0026-01-31 {}
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (ERROR
      (ERROR
        (UNEXPECTED '-')
        (UNEXPECTED ' '))
      (UNEXPECTED '0'))))(UNEXPECTED '-')(ERROR)
"#
    );
}

#[test]
fn test_revision_full() {
    parse_success_as!(
        r#"
module rev-test {
    revision 2026-01-30 {
        description "today is 2026-01-30";
        reference tree-sitter-yang.git;
    }
    revision 2026-01-31 {
        description "today is 2026-01-31";
        reference "foo@bar.baz";
    }
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (revision
      arg: (revision_date_str)
      (description
        arg: (string))
      (reference
        arg: (string
          (identifier))))
    (revision
      arg: (revision_date_str)
      (description
        arg: (string))
      (reference
        arg: (string)))))
"#
    );
}
