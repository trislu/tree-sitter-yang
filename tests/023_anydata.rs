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
    arg: (module_arg_str
      (identifier))
    (anydata_stmt
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
    arg: (module_arg_str
      (identifier))
    (anydata_stmt
      arg: (anydata_arg_str
        (identifier))
      (when_stmt
        arg: (string))
      (if_feature_stmt
        arg: (if_feature_arg_str
          (identifier)))
      (must_stmt
        arg: (must_expression))
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
