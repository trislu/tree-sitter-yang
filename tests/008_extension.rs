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
    arg: (module_arg_str
      (identifier))
    (extension_stmt
      arg: (extension_arg_str
        (identifier))
      (description_stmt
        arg: (description_arg_str))
      (argument_stmt
        arg: (argument_arg_str
          (identifier))))))
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
    arg: (module_arg_str
      (identifier))
    (extension_stmt
      arg: (extension_arg_str
        (identifier)))))
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
    arg: (module_arg_str
      (identifier))
    (extension_stmt
      arg: (extension_arg_str
        (identifier))
      (description_stmt
        arg: (description_arg_str))
      (argument_stmt
        arg: (argument_arg_str
          (identifier))
        (yin_element_stmt
          arg: (yin_element_arg_str
            (boolean))))
      (reference_stmt
        arg: (reference_arg_str))
      (status_stmt
        arg: (status_arg_str)))))
"#
    );
}
