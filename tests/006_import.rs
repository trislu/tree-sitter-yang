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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (import_stmt
      (import_keyword)
      arg: (import_arg_str
        (identifier))
      (prefix_stmt
        (prefix_keyword)
        arg: (prefix_arg_str
          (identifier))))))
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (import_stmt
      (import_keyword)
      arg: (import_arg_str
        (identifier))
      (prefix_stmt
        (prefix_keyword)
        arg: (prefix_arg_str
          (identifier)))
      (revision_date_stmt
        (revision_date_keyword)
        arg: (revision_date_arg_str
          (date_str)))
      (description_stmt
        (description_keyword)
        arg: (description_arg_str
          (quoted_string)))
      (reference_stmt
        (reference_keyword)
        arg: (reference_arg_str
          (quoted_string))))))
"#
    );
}
