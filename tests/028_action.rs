mod test_utils;

#[test]
fn test_action() {
    parse_success_as!(
        r#"
module test{
    container software {
        action activate-software-image {
            input {
                leaf image-name {
                    type string;
                }
            }
            output {
                leaf status {
                    type string;
                }
            }
        }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (container_stmt
      arg: (identifier)
      (action_stmt
        arg: (identifier)
        (input_stmt
          (leaf_stmt
            arg: (identifier)
            (type_stmt
              arg: (identifier))))
        (output_stmt
          (leaf_stmt
            arg: (identifier)
            (type_stmt
              arg: (identifier))))))))
        "#
    );
}

#[test]
fn test_action_full() {
    parse_success_as!(
        r#"
module test{
    container software {
        action activate-software-image {
            if-feature xyz;
            status deprecated;
            description "test";
            reference "test_action_full.com";
            typedef my-type {
                type uint32;
            }
            grouping nested-group {
                status current;
                description
                "nested-group is some nesty sh";
                reference "http://my-group/nested-group";
                typedef nested-type {
                    type uint32;
                }
            }
            input {
                leaf image-name {
                    type string;
                }
            }
            output {
                leaf status {
                    type string;
                }
            }
        }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (container_stmt
      arg: (identifier)
      (action_stmt
        arg: (identifier)
        (if_feature_stmt
          arg: (identifier))
        (status_stmt)
        (description
          arg: (qstring))
        (reference
          arg: (string))
        (typedef_stmt
          arg: (identifier)
          (type_stmt
            arg: (identifier)))
        (grouping_stmt
          arg: (identifier)
          (status_stmt)
          (description
            arg: (qstring))
          (reference
            arg: (string))
          (typedef_stmt
            arg: (identifier)
            (type_stmt
              arg: (identifier))))
        (input_stmt
          (leaf_stmt
            arg: (identifier)
            (type_stmt
              arg: (identifier))))
        (output_stmt
          (leaf_stmt
            arg: (identifier)
            (type_stmt
              arg: (identifier))))))))
        "#
    );
}
