mod test_utils;

#[test]
fn test_extension() {
    // the block can be presence
    parse_success_as!(
        r#"
module test {
    extension c-define {
        description
        "Takes as argument a name string.
        Makes the code generator use the given name in the
        #define.";
        argument "name";
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (extension_stmt
      arg: (identifier)
      (description_stmt
        arg: (qstring))
      (argument_stmt
        arg: (identifier)))))
"#
    );
    // the block can be omitted
    parse_success_as!(
        r#"
module test {
    extension bar ;
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (extension_stmt
      arg: (identifier))))
"#
    );
}

#[test]
fn test_extension_full() {
    parse_success_as!(
        r#"
module test {
    extension c-define {
        description
        "Takes as argument a name string.
        Makes the code generator use the given name in the
        #define.";
        argument "name" {
            yin-element "true";
        }
        reference "http://c-define.joke";
        status obsolete;
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (extension_stmt
      arg: (identifier)
      (description_stmt
        arg: (qstring))
      (argument_stmt
        arg: (identifier)
        (yin_element_stmt))
      (reference_stmt
        arg: (string))
      (status_stmt))))
"#
    );
}
