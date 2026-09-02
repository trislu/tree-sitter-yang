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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (leaf_stmt
      (leaf_keyword)
      arg: (leaf_arg_str
        (identifier))
      (type_stmt
        (type_keyword)
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (leaf_stmt
      (leaf_keyword)
      arg: (leaf_arg_str
        (identifier))
      (when_stmt
        (when_keyword)
        arg: (string
          (quoted_string)))
      (if_feature_stmt
        (if_feature_keyword)
        arg: (if_feature_arg_str
          (identifier)))
      (type_stmt
        (type_keyword)
        arg: (type_arg_str
          (identifier)))
      (units_stmt
        (units_keyword)
        arg: (units_arg_str
          (quoted_string)))
      (must_stmt
        (must_keyword)
        arg: (must_expression
          (quoted_string)))
      (default_stmt
        (default_keyword)
        arg: (default_arg_str))
      (config_stmt
        (config_keyword)
        arg: (boolean))
      (mandatory_stmt
        (mandatory_keyword)
        arg: (boolean))
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
          (quoted_string))))))
        "#
    );
}
