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
    arg: (identifier)
    (leaf_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)))))
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
    arg: (identifier)
    (leaf_stmt
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
      (config_stmt
        arg: (boolean))
      (mandatory_stmt
        arg: (boolean))
      (status_stmt)
      (description_stmt
        arg: (qstring))
      (reference_stmt
        arg: (string)))))
        "#
    );
}
