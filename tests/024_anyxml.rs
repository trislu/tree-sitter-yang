mod test_utils;

#[test]
fn test_anyxml() {
    parse_success_as!(
        r#"
module test{
    anyxml data;
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (anyxml_stmt
      arg: (identifier))))
        "#
    );
}

#[test]
fn test_anyxml_full() {
    parse_success_as!(
        r#"
module test{
    anyxml foo {
        when "what";
        if-feature xyz;
        must "be";
        config true;
        mandatory true;
        status current;
        description "test full leaf";
        reference "tests/024_anyxml.rs";
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (anyxml_stmt
      arg: (identifier)
      (when_stmt
        arg: (string))
      (if_feature_stmt
        arg: (identifier))
      (must_stmt
        arg: (must_expression))
      (config_stmt)
      (mandatory_stmt)
      (status_stmt)
      (description
        arg: (string))
      (reference
        arg: (string)))))
        "#
    );
}
