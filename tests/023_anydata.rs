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
    arg: (identifier)
    (anydata_stmt
      arg: (identifier))))
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
    arg: (identifier)
    (anydata_stmt
      arg: (identifier)
      (when_stmt
        arg: (string))
      (if_feature_stmt
        arg: (identifier))
      (must_stmt
        arg: (must_expression))
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
