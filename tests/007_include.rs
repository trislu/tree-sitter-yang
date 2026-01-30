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
    arg: (identifier)
    (include_stmt
      arg: (identifier))))
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
    arg: (identifier)
    (include_stmt
      arg: (identifier))))
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
    arg: (identifier)
    (include_stmt
      arg: (identifier)
      (revision_date
        arg: (date_str))
      (description
        arg: (string))
      (reference
        arg: (string)))))
"#
    );
}
