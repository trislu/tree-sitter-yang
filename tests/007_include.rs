mod test_utils;

#[test]
fn test_include() {
    // the block can be presence
    parse_success_as!(
        r#"
module test {
    include alice {
    }
}
    "#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (include_stmt
      (include_keyword)
      arg: (include_arg_str
        (identifier)))))
"#
    );
    // the block can also be omitted
    parse_success_as!(
        r#"
module test {
    include alice ;
}
    "#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (include_stmt
      (include_keyword)
      arg: (include_arg_str
        (identifier)))))
"#
    );
}

#[test]
fn test_include_full() {
    parse_success_as!(
        r#"
module test {
    include alice {
        revision-date 2000-01-01;
        description 'f
        u
        l
        l';
        reference "full-include@test";
    }
}
    "#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (include_stmt
      (include_keyword)
      arg: (include_arg_str
        (identifier))
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
