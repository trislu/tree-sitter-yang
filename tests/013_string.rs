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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (typedef_stmt
      (typedef_keyword)
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        (type_keyword)
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (typedef_stmt
      (typedef_keyword)
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        (type_keyword)
        arg: (type_arg_str
          (identifier))
        (length_stmt
          (length_keyword)
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (typedef_stmt
      (typedef_keyword)
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        (type_keyword)
        arg: (type_arg_str
          (identifier))
        (length_stmt
          (length_keyword)
          arg: (length_arg_str)
          (error_message_stmt
            (error_message_keyword)
            arg: (string
              (quoted_string))))))))
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (typedef_stmt
      (typedef_keyword)
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        (type_keyword)
        arg: (type_arg_str
          (identifier))
        (length_stmt
          (length_keyword)
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (typedef_stmt
      (typedef_keyword)
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        (type_keyword)
        arg: (type_arg_str
          (identifier))
        (pattern_stmt
          (pattern_keyword)
          arg: (pattern_arg_str
            (quoted_string)))))))
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (typedef_stmt
      (typedef_keyword)
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        (type_keyword)
        arg: (type_arg_str
          (identifier))
        (pattern_stmt
          (pattern_keyword)
          arg: (pattern_arg_str
            (quoted_string))
          (modifier_stmt
            (modifier_keyword)
            arg: (modifier_arg_str))
          (error_message_stmt
            (error_message_keyword)
            arg: (string
              (quoted_string))))))))
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (typedef_stmt
      (typedef_keyword)
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        (type_keyword)
        arg: (type_arg_str
          (identifier))
        (pattern_stmt
          (pattern_keyword)
          arg: (pattern_arg_str
            (quoted_string)))
        (pattern_stmt
          (pattern_keyword)
          arg: (pattern_arg_str
            (quoted_string)))))))
"#
    );
}

#[test]
fn test_concatenated_string() {
    parse_success_as!(
        r#"
module test-module {
    description "foo"
     + "/test-module";
}
    "#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (description_stmt
      (description_keyword)
      arg: (description_arg_str
        (quoted_string)
        (quoted_string)))))
"#
    );
}

#[test]
#[ignore = r#"as numeric rules introduced, the "UNEXPECTED" behavior is hard to predict."#]
fn test_string_invalid_cases() {
    // todo
}
