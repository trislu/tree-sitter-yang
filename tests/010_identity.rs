mod test_utils;

#[test]
fn test_identity() {
    parse_success_as!(
        r#"
module test {
    yang-version 1.1;
    prefix foo;
    namespace "urn:example:foo";
    identity foo;
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
    (identity_stmt
      (identity_keyword)
      arg: (identity_arg_str
        (identifier)))))
"#
    );
}

#[test]
fn test_identity_full() {
    parse_success_as!(
        r#"
module test {
    yang-version 1.1;
    prefix foo;
    namespace "urn:example:foo";
    identity foo {
        base x:y;
        status deprecated;
        description
            "just a simple foo feature";
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
    (identity_stmt
      (identity_keyword)
      arg: (identity_arg_str
        (identifier))
      (base_stmt
        (base_keyword)
        arg: (base_arg_str
          (prefix
            (identifier))
          (identifier)))
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
