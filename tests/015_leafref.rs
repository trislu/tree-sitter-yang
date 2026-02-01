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
    arg: (identifier)
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)))
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (path_stmt
          arg: (node_identifier
            (identifier))
          arg: (node_identifier
            (identifier)))))))
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
    arg: (identifier)
    (typedef_stmt
      arg: (identifier)
      (description_stmt
        arg: (qstring))
      (type_stmt
        arg: (identifier)
        (path_stmt
          arg: (node_identifier
            (identifier))
          arg: (node_identifier
            (identifier))
          arg: (node_identifier
            (identifier)))
        (require_instance_stmt)))))
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
