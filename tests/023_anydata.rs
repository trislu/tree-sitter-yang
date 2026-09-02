mod test_utils;

#[test]
fn test_anydata() {
    parse_success_as!(
        r#"
module test{
    anydata data;
}
    "#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (anydata_stmt
      (anydata_keyword)
      arg: (anydata_arg_str
        (identifier)))))
        "#
    );
}

#[test]
fn test_anydata_full() {
    parse_success_as!(
        r#"
module test{
    anydata foo {
        when "what";
        if-feature xyz;
        must "be";
        config true;
        mandatory true;
        status current;
        description "test full leaf";
        reference "tests/023_anydata.rs";
    }
}
    "#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (anydata_stmt
      (anydata_keyword)
      arg: (anydata_arg_str
        (identifier))
      (when_stmt
        (when_keyword)
        arg: (string
          (quoted_string)))
      (if_feature_stmt
        (if_feature_keyword)
        arg: (if_feature_arg_str
          (identifier)))
      (must_stmt
        (must_keyword)
        arg: (must_expression
          (quoted_string)))
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
