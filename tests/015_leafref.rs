mod test_utils;

#[test]
fn test_leafref_basic() {
    parse_success_as!(
        r#"
module test{
    typedef parent-leaf {
        type string;
    }
    typedef child-leaf {
        type leafref {
            path "/test/parent-leaf";
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
          (identifier))))
    (typedef_stmt
      (typedef_keyword)
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        (type_keyword)
        arg: (type_arg_str
          (identifier))
        (path_stmt
          (path_keyword)
          arg: (path_arg_str
            (quoted_string)))))))
        "#
    );
}

#[test]
fn test_leafref_with_require_instance() {
    parse_success_as!(
        r#"
module test{
    typedef leafref-desc {
        description "Reference to the name of an active device";
        type leafref {    
            path "/test/device/name";
            require-instance true;
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
      (description_stmt
        (description_keyword)
        arg: (description_arg_str
          (quoted_string)))
      (type_stmt
        (type_keyword)
        arg: (type_arg_str
          (identifier))
        (path_stmt
          (path_keyword)
          arg: (path_arg_str
            (quoted_string)))
        (require_instance_stmt
          (require_instance_keyword)
          arg: (require_instance_arg_str
            (boolean)))))))
"#
    );
}

#[test]
#[ignore = "leafref validation should be semantic"]
fn test_leafref_invalid_cases() {
    /*
    module test{
        leaf invalid-ref {
            type leafref {
                path "/test/non-existent-leaf";
            }
        }
    }
    */
}
