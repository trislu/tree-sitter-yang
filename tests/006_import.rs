mod test_utils;

#[test]
fn test_import() {
    parse_success_as!(
        r#"
module test {
    import alice {
        prefix a;
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (import_stmt
      arg: (identifier)
      (prefix_stmt
        arg: (identifier)))))
"#
    );
}

#[test]
fn test_import_full() {
    parse_success_as!(
        r#"
module test {
    import alice {
        prefix a{   }
        revision-date 2000-01-01;
        description 'f
        u
        l
        l';
        reference "full-import@test";
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (import_stmt
      arg: (identifier)
      (prefix_stmt
        arg: (identifier))
      (revision_date_stmt
        arg: (date_str))
      (description_stmt
        arg: (qstring))
      (reference_stmt
        arg: (string)))))
"#
    );
}
