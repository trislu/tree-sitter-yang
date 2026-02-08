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
    arg: (module_arg_str
      (identifier))
    (include_stmt
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
    arg: (module_arg_str
      (identifier))
    (include_stmt
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
    arg: (module_arg_str
      (identifier))
    (include_stmt
      arg: (include_arg_str
        (identifier))
      (revision_date_stmt
        arg: (revision_date_arg_str
          (date_str)))
      (description_stmt
        arg: (description_arg_str))
      (reference_stmt
        arg: (reference_arg_str)))))
"#
    );
}
