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
    arg: (module_arg_str
      (identifier))
    (yang_version_stmt
      arg: (yang_version_arg_str))
    (prefix_stmt
      arg: (prefix_arg_str
        (identifier)))
    (namespace_stmt
      arg: (namespace_arg_str))
    (feature_stmt
      arg: (feature_arg_str
        (identifier)))))
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
        if-feature xy:z{}
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
    arg: (module_arg_str
      (identifier))
    (yang_version_stmt
      arg: (yang_version_arg_str))
    (prefix_stmt
      arg: (prefix_arg_str
        (identifier)))
    (namespace_stmt
      arg: (namespace_arg_str))
    (feature_stmt
      arg: (feature_arg_str
        (identifier))
      (if_feature_stmt
        arg: (if_feature_arg_str
          (identifier)))
      (if_feature_stmt
        arg: (if_feature_arg_str
          (identifier)))
      (if_feature_stmt
        arg: (if_feature_arg_str
          (prefix
            (identifier))
          (identifier)))
      (description_stmt
        arg: (description_arg_str))
      (status_stmt
        arg: (status_arg_str))
      (reference_stmt
        arg: (reference_arg_str)))))
"#
    );
}
