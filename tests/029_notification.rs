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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (container_stmt
      (container_keyword)
      arg: (container_arg_str
        (identifier))
      (notification_stmt
        (notification_keyword)
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (container_stmt
      (container_keyword)
      arg: (container_arg_str
        (identifier))
      (notification_stmt
        (notification_keyword)
        arg: (notification_arg_str
          (identifier))
        (if_feature_stmt
          (if_feature_keyword)
          arg: (if_feature_arg_str
            (identifier)))
        (must_stmt
          (must_keyword)
          arg: (must_expression
            (quoted_string)))
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
        (container_stmt
          (container_keyword)
          arg: (container_arg_str
            (identifier)))
        (list_stmt
          (list_keyword)
          arg: (list_arg_str
            (identifier)))
        (leaf_stmt
          (leaf_keyword)
          arg: (leaf_arg_str
            (identifier))
          (type_stmt
            (type_keyword)
            arg: (type_arg_str
              (identifier))))
        (leaf_list_stmt
          (leaf_list_keyword)
          arg: (leaf_list_arg_str
            (identifier))
          (type_stmt
            (type_keyword)
            arg: (type_arg_str
              (identifier))))
        (choice_stmt
          (choice_keyword)
          arg: (choice_arg_str
            (identifier)))
        (anydata_stmt
          (anydata_keyword)
          arg: (anydata_arg_str
            (identifier)))
        (anyxml_stmt
          (anyxml_keyword)
          arg: (anyxml_arg_str
            (identifier)))
        (uses_stmt
          (uses_keyword)
          arg: (uses_arg_str
            (identifier)))))))
        "#
    );
}
