mod test_utils;

#[test]
fn test_feature() {
    parse_success_as!(
        r#"
module test {
    yang-version 1.1;
    prefix foo;
    namespace "urn:example:foo";
    feature foo;
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (yang_version)
    (prefix_stmt
      arg: (identifier))
    (namespace_stmt
      arg: (uri_str))
    (feature_stmt
      arg: (identifier))))
"#
    );
}

#[test]
fn test_feature_full() {
    parse_success_as!(
        r#"
module test {
    yang-version 1.1;
    prefix foo;
    namespace "urn:example:foo";
    feature foo {
        if-feature x;
        if-feature y{}
        description
            "just a simple foo feature";
        status current;
        reference "http://foo";
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (yang_version)
    (prefix_stmt
      arg: (identifier))
    (namespace_stmt
      arg: (uri_str))
    (feature_stmt
      arg: (identifier)
      (if_feature_stmt
        arg: (identifier))
      (if_feature_stmt
        arg: (identifier))
      (description
        arg: (string))
      (status_stmt)
      (reference
        arg: (string)))))
"#
    );
}
