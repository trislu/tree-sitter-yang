mod test_utils;

#[test]
fn test_typedef() {
    parse_success_as!(
        r#"
module test {
    yang-version 1.1;
    prefix foo;
    namespace "urn:example:foo";
    typedef bar {
        status current;
        description "dry martini, shaken, not stirred";
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
    (typedef_stmt
      arg: (identifier)
      (status_stmt)
      (description
        arg: (string)))))
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
    arg: (identifier)
    (yang_version)
    (prefix_stmt
      arg: (identifier))
    (namespace_stmt
      arg: (uri_str))
    (identity_stmt
      arg: (identifier)
      (base_stmt
        arg: (prefix
          (identifier))
        arg: (identifier))
      (status_stmt)
      (description
        arg: (string))
      (reference
        arg: (string)))))
"#
    );
}
