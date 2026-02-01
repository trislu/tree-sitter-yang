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
    arg: (identifier)
    (rpc_stmt
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
            arg: (identifier)))))))
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
    arg: (identifier)
    (rpc_stmt
      arg: (identifier)
      (if_feature_stmt
        arg: (identifier))
      (status_stmt)
      (description_stmt
        arg: (qstring))
      (reference_stmt
        arg: (string))
      (typedef_stmt
        arg: (identifier)
        (type_stmt
          arg: (identifier)))
      (grouping_stmt
        arg: (identifier)
        (status_stmt)
        (description_stmt
          arg: (qstring))
        (reference_stmt
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
            arg: (identifier)))))))
        "#
    );
}
