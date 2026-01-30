mod test_utils;

#[test]
fn test_single_revision() {
    // the block can be presence
    parse_success_as!(
        r#"
module rev-test {
    revision 2026-01-30 {}
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (revision_stmt
      arg: (date_str))))
"#
    );
    // the block can be omitted
    parse_success_as!(
        r#"
module rev-test {
    revision 2026-01-30;
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (revision_stmt
      arg: (date_str))))
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
  (module_stmt
    arg: (identifier)
    (revision_stmt
      arg: (date_str))
    (revision_stmt
      arg: (date_str))
    (revision_stmt
      arg: (date_str))))
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
  (module_stmt
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
  (module_stmt
    arg: (identifier)
    (revision_stmt
      arg: (date_str)
      (description
        arg: (string))
      (reference
        arg: (string
          (identifier))))
    (revision_stmt
      arg: (date_str)
      (description
        arg: (string))
      (reference
        arg: (string)))))
"#
    );
}
