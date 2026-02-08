mod test_utils;

#[test]
fn test_notification() {
    parse_success_as!(
        r#"
module test{
    container software {
        notification activate-software {
            
        }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (module_arg_str
      (identifier))
    (container_stmt
      arg: (container_arg_str
        (identifier))
      (notification_stmt
        arg: (notification_arg_str
          (identifier))))))
        "#
    );
}

#[test]
fn test_notifiation_full() {
    parse_success_as!(
        r#"
module test{
    container software {
        notification activate-software {
            if-feature xyz;
            must "what";
            status obsolete;
            description "test notification";
            reference "tests/029_notification.rs";
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
            container foo{}
            list bar {}
            leaf baz {type string;}
            leaf-list qux {type string;}
            choice what {}
            anydata data;
            anyxml xml;
            uses bar;
        }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (module_arg_str
      (identifier))
    (container_stmt
      arg: (container_arg_str
        (identifier))
      (notification_stmt
        arg: (notification_arg_str
          (identifier))
        (if_feature_stmt
          arg: (if_feature_arg_str
            (identifier)))
        (must_stmt
          arg: (must_expression))
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
        (container_stmt
          arg: (container_arg_str
            (identifier)))
        (list_stmt
          arg: (list_arg_str
            (identifier)))
        (leaf_stmt
          arg: (leaf_arg_str
            (identifier))
          (type_stmt
            arg: (type_arg_str
              (identifier))))
        (leaf_list_stmt
          arg: (leaf_list_arg_str
            (identifier))
          (type_stmt
            arg: (type_arg_str
              (identifier))))
        (choice_stmt
          arg: (choice_arg_str
            (identifier)))
        (anydata_stmt
          arg: (anydata_arg_str
            (identifier)))
        (anyxml_stmt
          arg: (anyxml_arg_str
            (identifier)))
        (uses_stmt
          arg: (uses_arg_str
            (identifier)))))))
        "#
    );
}
