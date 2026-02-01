mod test_utils;

#[test]
fn test_single_range() {
    parse_success_as!(
        r#"
module test {
    yang-version 1.1;
    prefix foo;
    namespace "urn:example:foo";
    typedef my-type {
        type int32 {
            range "11..100";
        }
    }
}
"#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (yang_version_stmt)
    (prefix_stmt
      arg: (identifier))
    (namespace_stmt
      arg: (uri_str))
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (range_stmt
          arg: (integer_value)
          arg: (integer_value))))))
"#
    );
}

#[test]
fn test_multiple_range() {
    parse_success_as!(
        r#"
module test {
    yang-version 1.1;
    prefix foo;
    namespace "urn:example:foo";
    typedef my-base-int32-type {
        type int32 {
            range "1..4 | 10..20";
        }
    }
}
"#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (yang_version_stmt)
    (prefix_stmt
      arg: (identifier))
    (namespace_stmt
      arg: (uri_str))
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (range_stmt
          arg: (integer_value)
          arg: (integer_value)
          arg: (integer_value)
          arg: (integer_value))))))
"#
    );
}

#[test]
fn test_from_min_range() {
    parse_success_as!(
        r#"
module test {
    yang-version 1.1;
    prefix foo;
    namespace "urn:example:foo";
    typedef my-base-int32-type {
        type int32 {
            range "1..4 | 10..20";
        }
    }
    typedef my-type1 {
        type my-base-int32-type {
            // legal range restriction
            range "min..3"; // 1..3
        }
    }
}
"#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (yang_version_stmt)
    (prefix_stmt
      arg: (identifier))
    (namespace_stmt
      arg: (uri_str))
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (range_stmt
          arg: (integer_value)
          arg: (integer_value)
          arg: (integer_value)
          arg: (integer_value))))
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (comment)
        (range_stmt
          arg: (integer_value))
        (comment)))))
"#
    );
}

#[test]
fn test_to_max_range() {
    parse_success_as!(
        r#"
module test {
    yang-version 1.1;
    prefix foo;
    namespace "urn:example:foo";
    typedef my-base-int32-type {
        type int32 {
            range "1..4 | 10..20";
        }
    }
    typedef my-type1 {
        type my-base-int32-type {
            // legal range restriction
            range "11..max"; // 11..20
        }
    }
}
"#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (yang_version_stmt)
    (prefix_stmt
      arg: (identifier))
    (namespace_stmt
      arg: (uri_str))
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (range_stmt
          arg: (integer_value)
          arg: (integer_value)
          arg: (integer_value)
          arg: (integer_value))))
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (comment)
        (range_stmt
          arg: (integer_value))
        (comment)))))
"#
    );
}

// we dont have an "error-app-tag" example in both rfc6020 & rfc7950, strange
#[test]
fn test_range_error_msg() {
    parse_success_as!(
        r#"
module test {
    yang-version 1.1;
    prefix foo;
    namespace "urn:example:foo";
    typedef my-type {
        type int32 {
            range "11..100" {
                error-message "my-base-int32-type  MTU must be 11..100";
            }
        }
    }
}
"#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (yang_version_stmt)
    (prefix_stmt
      arg: (identifier))
    (namespace_stmt
      arg: (uri_str))
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (range_stmt
          arg: (integer_value)
          arg: (integer_value)
          (error_message_stmt
            arg: (string)))))))
"#
    );
}

//
