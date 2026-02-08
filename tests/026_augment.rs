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
    arg: (module_arg_str
      (identifier))
    (augment_stmt
      arg: (augment_arg_str
        (node_identifier
          (identifier)
          (identifier)))
      (if_feature_stmt
        arg: (if_feature_arg_str
          (identifier)))
      (leaf_stmt
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
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
    arg: (module_arg_str
      (identifier))
    (augment_stmt
      arg: (augment_arg_str
        (node_identifier
          (identifier)
          (identifier)))
      (when_stmt
        arg: (string))
      (if_feature_stmt
        arg: (if_feature_arg_str
          (identifier)))
      (status_stmt
        arg: (status_arg_str))
      (description_stmt
        arg: (description_arg_str))
      (reference_stmt
        arg: (reference_arg_str))
      (leaf_stmt
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          arg: (type_arg_str
            (identifier))))
      (case_stmt
        arg: (case_arg_str
          (identifier))))))
        "#
    );
}
