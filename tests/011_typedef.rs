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
    arg: (module_arg_str
      (identifier))
    (yang_version_stmt
      arg: (yang_version_arg_str))
    (prefix_stmt
      arg: (prefix_arg_str
        (identifier)))
    (namespace_stmt
      arg: (namespace_arg_str))
    (typedef_stmt
      arg: (typedef_arg_str
        (identifier))
      (units_stmt
        arg: (units_arg_str))
      (default_stmt
        arg: (default_arg_str))
      (status_stmt
        arg: (status_arg_str))
      (description_stmt
        arg: (description_arg_str))
      (reference_stmt
        arg: (reference_arg_str)))))
"#
    );
}
