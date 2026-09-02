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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (container_stmt
      (container_keyword)
      arg: (container_arg_str
        (identifier))
      (action_stmt
        (action_keyword)
        arg: (action_arg_str
          (identifier))
        (input_stmt
          (input_keyword)
          (leaf_stmt
            (leaf_keyword)
            arg: (leaf_arg_str
              (identifier))
            (type_stmt
              (type_keyword)
              arg: (type_arg_str
                (identifier)))))
        (output_stmt
          (output_keyword)
          (leaf_stmt
            (leaf_keyword)
            arg: (leaf_arg_str
              (identifier))
            (type_stmt
              (type_keyword)
              arg: (type_arg_str
                (identifier)))))))))
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (container_stmt
      (container_keyword)
      arg: (container_arg_str
        (identifier))
      (action_stmt
        (action_keyword)
        arg: (action_arg_str
          (identifier))
        (if_feature_stmt
          (if_feature_keyword)
          arg: (if_feature_arg_str
            (identifier)))
        (status_stmt
          (status_keyword)
          arg: (status_arg_str))
        (description_stmt
          (description_keyword)
          arg: (description_arg_str
            (quoted_string)))
        (reference_stmt
          (reference_keyword)
          arg: (reference_arg_str
            (quoted_string)))
        (typedef_stmt
          (typedef_keyword)
          arg: (typedef_arg_str
            (identifier))
          (type_stmt
            (type_keyword)
            arg: (type_arg_str
              (identifier))))
        (grouping_stmt
          (grouping_keyword)
          arg: (grouping_arg_str
            (identifier))
          (status_stmt
            (status_keyword)
            arg: (status_arg_str))
          (description_stmt
            (description_keyword)
            arg: (description_arg_str
              (quoted_string)))
          (reference_stmt
            (reference_keyword)
            arg: (reference_arg_str
              (quoted_string)))
          (typedef_stmt
            (typedef_keyword)
            arg: (typedef_arg_str
              (identifier))
            (type_stmt
              (type_keyword)
              arg: (type_arg_str
                (identifier)))))
        (input_stmt
          (input_keyword)
          (leaf_stmt
            (leaf_keyword)
            arg: (leaf_arg_str
              (identifier))
            (type_stmt
              (type_keyword)
              arg: (type_arg_str
                (identifier)))))
        (output_stmt
          (output_keyword)
          (leaf_stmt
            (leaf_keyword)
            arg: (leaf_arg_str
              (identifier))
            (type_stmt
              (type_keyword)
              arg: (type_arg_str
                (identifier)))))))))
        "#
    );
}
