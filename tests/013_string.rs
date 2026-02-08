mod test_utils;

#[test]
fn test_string_basic() {
    parse_success_as!(
        r#"
module test{
    typedef test-string {
        type string;
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
          (identifier))))))
        "#
    );
}

#[test]
fn test_string_length_restriction() {
    parse_success_as!(
        r#"
module test{
    typedef test-length {
        type string {
            length 10;
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
        (length_stmt
          arg: (length_arg_str))))))
        "#
    );

    parse_success_as!(
        r#"
module test{
    typedef test-length-range {
        type string {
            length min..max {
                error-message "Length out of range";
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
        (length_stmt
          arg: (length_arg_str)
          (error_message_stmt
            arg: (string)))))))
        "#
    );

    parse_success_as!(
        r#"
module test{
    typedef test-length-multi {
        type string {
            length 1..5 | 10..20;
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
        (length_stmt
          arg: (length_arg_str))))))
"#
    );
}

#[test]
fn test_string_pattern_restriction() {
    parse_success_as!(
        r#"
module test{
    typedef test-pattern {
        type string {
            pattern "^[a-zA-Z0-9]+$";
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
        (pattern_stmt
          arg: (pattern_arg_str))))))
"#
    );

    // pattern + modifier (invert-match, RFC7950)
    parse_success_as!(
        r#"
module test{
    typedef test-pattern-modifier {
        type string {
            pattern "^invalid$" {
                modifier invert-match;
                error-message "Must not match invalid pattern";
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
        (pattern_stmt
          arg: (pattern_arg_str)
          (modifier_stmt
            arg: (modifier_arg_str))
          (error_message_stmt
            arg: (string)))))))
"#
    );

    parse_success_as!(
        r#"
module test{
        typedef test-pattern-multi {
            type string {
                pattern "^foo";
                pattern "bar$";
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
        (pattern_stmt
          arg: (pattern_arg_str))
        (pattern_stmt
          arg: (pattern_arg_str))))))
"#
    );
}

#[test]
#[ignore = r#"as numeric rules introduced, the "UNEXPECTED" behavior is hard to predict."#]
fn test_string_invalid_cases() {
    // todo
}
