mod test_utils;

#[test]
fn test_augment() {
    parse_success_as!(
        r#"
module test{
    rpc activate-software-image {
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
    "#,
        r#"
(yang
  (module_stmt
    arg: (module_arg_str
      (identifier))
    (rpc_stmt
      arg: (rpc_arg_str
        (identifier))
      (input_stmt
        (leaf_stmt
          arg: (leaf_arg_str
            (identifier))
          (type_stmt
            arg: (type_arg_str
              (identifier)))))
      (output_stmt
        (leaf_stmt
          arg: (leaf_arg_str
            (identifier))
          (type_stmt
            arg: (type_arg_str
              (identifier))))))))
        "#
    );
}

#[test]
fn test_rpc_full() {
    parse_success_as!(
        r#"
module test{
    rpc activate-software-image {
        if-feature xyz;
        status deprecated;
        description "test";
        reference "test_rpc_full.com";
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
    "#,
        r#"
(yang
  (module_stmt
    arg: (module_arg_str
      (identifier))
    (rpc_stmt
      arg: (rpc_arg_str
        (identifier))
      (if_feature_stmt
        arg: (if_feature_arg_str
          (identifier)))
      (status_stmt
        arg: (status_arg_str))
      (description_stmt
        arg: (description_arg_str))
      (reference_stmt
        arg: (reference_arg_str))
      (typedef_stmt
        arg: (typedef_arg_str
          (identifier))
        (type_stmt
          arg: (type_arg_str
            (identifier))))
      (grouping_stmt
        arg: (grouping_arg_str
          (identifier))
        (status_stmt
          arg: (status_arg_str))
        (description_stmt
          arg: (description_arg_str))
        (reference_stmt
          arg: (reference_arg_str))
        (typedef_stmt
          arg: (typedef_arg_str
            (identifier))
          (type_stmt
            arg: (type_arg_str
              (identifier)))))
      (input_stmt
        (leaf_stmt
          arg: (leaf_arg_str
            (identifier))
          (type_stmt
            arg: (type_arg_str
              (identifier)))))
      (output_stmt
        (leaf_stmt
          arg: (leaf_arg_str
            (identifier))
          (type_stmt
            arg: (type_arg_str
              (identifier))))))))
        "#
    );
}
