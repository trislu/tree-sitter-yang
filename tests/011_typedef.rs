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
        units 'shot';
        default "gin";
        status current;
        description "dry martini, shaken, not stirred";
        reference "JamesBond";
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
      (units_stmt
        arg: (string))
      (default_stmt
        arg: (string))
      (status_stmt)
      (description
        arg: (qstring))
      (reference
        arg: (string)))))
"#
    );
}
