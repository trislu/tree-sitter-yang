mod test_utils;

#[test]
fn test_leaflist() {
    parse_success_as!(
        r#"
module test{
    leaf-list foo {
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
    (leaf_list_stmt
      (leaf_list_keyword)
      arg: (leaf_list_arg_str
        (identifier))
      (type_stmt
        (type_keyword)
        arg: (type_arg_str
          (identifier))))))
        "#
    );
}

#[test]
fn test_leaf_list_full() {
    parse_success_as!(
        r#"
module test{
    leaf-list foo {
        when "what";
        if-feature xyz;
        type uint32;
        units "sec";
        must "be";
        default 123;
        config true;
        min-elements 2;
        max-elements 100;
        ordered-by system;
        status current;
        description "test full leaf-list";
        reference "tests/019_leaf_list.rs";
    }
}
    "#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (leaf_list_stmt
      (leaf_list_keyword)
      arg: (leaf_list_arg_str
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
      (min_elements_stmt
        (min_elements_keyword))
      (max_elements_stmt
        (max_elements_keyword))
      (ordered_by_stmt
        (ordered_by_keyword))
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
