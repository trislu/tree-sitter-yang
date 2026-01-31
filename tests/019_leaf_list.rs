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
    arg: (identifier)
    (leaflist_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)))))
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
    arg: (identifier)
    (leaflist_stmt
      arg: (identifier)
      (when_stmt
        arg: (string))
      (if_feature_stmt
        arg: (identifier))
      (type_stmt
        arg: (identifier))
      (units_stmt
        arg: (string))
      (must_stmt
        arg: (must_expression))
      (default_stmt
        arg: (integer_value))
      (config_stmt)
      (min_elements_stmt)
      (max_elements_stmt)
      (ordered_by_stmt)
      (status_stmt)
      (description
        arg: (string))
      (reference
        arg: (string)))))
        "#
    );
}
