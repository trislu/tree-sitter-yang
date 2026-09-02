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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (yang_version_stmt
      (yang_version_keyword)
      arg: (yang_version_arg_str))
    (prefix_stmt
      (prefix_keyword)
      arg: (prefix_arg_str
        (identifier)))
    (namespace_stmt
      (namespace_keyword)
      arg: (namespace_arg_str))
    (feature_stmt
      (feature_keyword)
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (yang_version_stmt
      (yang_version_keyword)
      arg: (yang_version_arg_str))
    (prefix_stmt
      (prefix_keyword)
      arg: (prefix_arg_str
        (identifier)))
    (namespace_stmt
      (namespace_keyword)
      arg: (namespace_arg_str))
    (feature_stmt
      (feature_keyword)
      arg: (feature_arg_str
        (identifier))
      (if_feature_stmt
        (if_feature_keyword)
        arg: (if_feature_arg_str
          (identifier)))
      (if_feature_stmt
        (if_feature_keyword)
        arg: (if_feature_arg_str
          (identifier)))
      (if_feature_stmt
        (if_feature_keyword)
        arg: (if_feature_arg_str
          (prefix
            (identifier))
          (identifier)))
      (description_stmt
        (description_keyword)
        arg: (description_arg_str
          (quoted_string)))
      (status_stmt
        (status_keyword)
        arg: (status_arg_str))
      (reference_stmt
        (reference_keyword)
        arg: (reference_arg_str
          (quoted_string))))))
"#
    );
}
