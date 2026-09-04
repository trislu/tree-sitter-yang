mod test_utils;

#[test]
fn test_augment() {
    parse_success_as!(
        r#"
module test{
    augment "/b:x" {
        if-feature foo;
        leaf y {
            type myenum;
        }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (augment_stmt
      (augment_keyword)
      arg: (augment_arg_str
        (quoted_string))
      (if_feature_stmt
        (if_feature_keyword)
        arg: (if_feature_arg_str
          (identifier)))
      (leaf_stmt
        (leaf_keyword)
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          (type_keyword)
          arg: (type_arg_str
            (identifier)))))))
        "#
    );
}

#[test]
fn test_augment_full() {
    parse_success_as!(
        r#"
module test{
    augment "/b:x" {
        when "xyz";
        if-feature foo;
        status deprecated;
        description "test";
        reference "test_augment_full.com";
        leaf y {
            type myenum;
        }
        case bar {}
    }
}
    "#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (augment_stmt
      (augment_keyword)
      arg: (augment_arg_str
        (quoted_string))
      (when_stmt
        (when_keyword)
        arg: (string
          (quoted_string)))
      (if_feature_stmt
        (if_feature_keyword)
        arg: (if_feature_arg_str
          (identifier)))
      (status_stmt
        (status_keyword)
        arg: (status_arg_str))
      (description_stmt
        (description_keyword)
        arg: (description_arg_str
          (quoted_string)))
      (reference_stmt
        (reference_keyword)
        arg: (reference_arg_str
          (quoted_string)))
      (leaf_stmt
        (leaf_keyword)
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          (type_keyword)
          arg: (type_arg_str
            (identifier))))
      (case_stmt
        (case_keyword)
        arg: (case_arg_str
          (identifier))))))
        "#
    );
}
