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
    arg: (identifier)
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)))))
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
    arg: (identifier)
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (length_stmt)))))
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
    arg: (identifier)
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (length_stmt
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
    arg: (identifier)
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (length_stmt)))))
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
    arg: (identifier)
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (pattern_stmt
          arg: (string))))))
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
    arg: (identifier)
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (pattern_stmt
          arg: (string)
          (modifier_stmt)
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
    arg: (identifier)
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (pattern_stmt
          arg: (string))
        (pattern_stmt
          arg: (string))))))
"#
    );
}

#[test]
#[ignore = r#"as numeric rules introduced, the "UNEXPECTED" behavior is hard to predict."#]
fn test_string_invalid_cases() {
    // todo
}
