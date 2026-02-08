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
    arg: (module_arg_str
      (identifier))
    (yang_version_stmt
      arg: (yang_version_arg_str))
    (prefix_stmt
      arg: (prefix_arg_str
        (identifier)))
    (namespace_stmt
      arg: (namespace_arg_str))
    (identity_stmt
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
    arg: (module_arg_str
      (identifier))
    (yang_version_stmt
      arg: (yang_version_arg_str))
    (prefix_stmt
      arg: (prefix_arg_str
        (identifier)))
    (namespace_stmt
      arg: (namespace_arg_str))
    (identity_stmt
      arg: (identity_arg_str
        (identifier))
      (base_stmt
        arg: (base_arg_str
          (prefix
            (identifier))
          (identifier)))
      (status_stmt
        arg: (status_arg_str))
      (description_stmt
        arg: (description_arg_str))
      (reference_stmt
        arg: (reference_arg_str)))))
"#
    );
}
