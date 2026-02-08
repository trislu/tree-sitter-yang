mod test_utils;

#[test]
fn test_leaf() {
    parse_success_as!(
        r#"
module test{
    leaf foo {
        type uint32;
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (module_arg_str
      (identifier))
    (leaf_stmt
      arg: (leaf_arg_str
        (identifier))
      (type_stmt
        arg: (type_arg_str
          (identifier))))))
        "#
    );
}

#[test]
fn test_leaf_full() {
    parse_success_as!(
        r#"
module test{
    leaf foo {
        when "what";
        if-feature xyz;
        type uint32;
        units "sec";
        must "be";
        default 123;
        config true;
        mandatory true;
        status current;
        description "test full leaf";
        reference "tests/018_leaf.rs";
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (module_arg_str
      (identifier))
    (leaf_stmt
      arg: (leaf_arg_str
        (identifier))
      (when_stmt
        arg: (string))
      (if_feature_stmt
        arg: (if_feature_arg_str
          (identifier)))
      (type_stmt
        arg: (type_arg_str
          (identifier)))
      (units_stmt
        arg: (units_arg_str))
      (must_stmt
        arg: (must_expression))
      (default_stmt
        arg: (default_arg_str))
      (config_stmt
        arg: (boolean))
      (mandatory_stmt
        arg: (boolean))
      (status_stmt
        arg: (status_arg_str))
      (description_stmt
        arg: (description_arg_str))
      (reference_stmt
        arg: (reference_arg_str)))))
        "#
    );
}
