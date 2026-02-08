mod test_utils;

#[test]
fn test_enum_basic() {
    parse_success_as!(
        r#"
module test{
    typedef test-enum {
        type enumeration {
            enum up;
            enum down;
        }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (module_arg_str
      (identifier))
    (typedef_stmt
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        arg: (type_arg_str
          (identifier))
        (enum_stmt
          arg: (enum_arg_str
            (string
              (identifier))))
        (enum_stmt
          arg: (enum_arg_str
            (string
              (identifier))))))))
        "#
    );
}

#[test]
fn test_enum_with_value() {
    parse_success_as!(
        r#"
module test{
    typedef test-enum-value {
        type enumeration {
            enum enabled {
                value 1;
            }
            enum disabled {
                value 0;
            }
        }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (module_arg_str
      (identifier))
    (typedef_stmt
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        arg: (type_arg_str
          (identifier))
        (enum_stmt
          arg: (enum_arg_str
            (string
              (identifier)))
          (value_stmt
            arg: (value_arg_str
              (integer_value))))
        (enum_stmt
          arg: (enum_arg_str
            (string
              (identifier)))
          (value_stmt
            arg: (value_arg_str
              (integer_value))))))))
        "#
    );
}

#[test]
fn test_enum_with_description() {
    parse_success_as!(
        r#"
module test{
    typedef test-enum-desc {
        type enumeration {
            enum running {
                description "Service is running";
                value 2;
            }
            enum stopped {
                description "Service is stopped";
                value 1;
            }
            enum error {
                description "Service has error";
                value 0;
            }
        }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (module_arg_str
      (identifier))
    (typedef_stmt
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        arg: (type_arg_str
          (identifier))
        (enum_stmt
          arg: (enum_arg_str
            (string
              (identifier)))
          (description_stmt
            arg: (description_arg_str))
          (value_stmt
            arg: (value_arg_str
              (integer_value))))
        (enum_stmt
          arg: (enum_arg_str
            (string
              (identifier)))
          (description_stmt
            arg: (description_arg_str))
          (value_stmt
            arg: (value_arg_str
              (integer_value))))
        (enum_stmt
          arg: (enum_arg_str
            (string
              (identifier)))
          (description_stmt
            arg: (description_arg_str))
          (value_stmt
            arg: (value_arg_str
              (integer_value))))))))
"#
    );
}

#[test]
#[ignore = "enum invalid cases need to be implemented with numeric rules check"]
fn test_enum_invalid_cases() {
    // todo: introduce syntatic check error macro
    /*
    module test{
        typedef test-invalid-enum {
            type enumeration {
                enum dup-value { value 5; }
                enum dup-value { value 5; }
            }
        }
    }
    */
}
