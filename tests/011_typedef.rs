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
    (typedef_stmt
      (typedef_keyword)
      arg: (typedef_arg_str
        (identifier))
      (units_stmt
        (units_keyword)
        arg: (units_arg_str
          (quoted_string)))
      (default_stmt
        (default_keyword)
        arg: (default_arg_str
          (quoted_string)))
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
