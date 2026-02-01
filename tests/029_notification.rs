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
    arg: (identifier)
    (container_stmt
      arg: (identifier)
      (notification_stmt
        arg: (identifier)))))
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
    arg: (identifier)
    (container_stmt
      arg: (identifier)
      (notification_stmt
        arg: (identifier)
        (if_feature_stmt
          arg: (identifier))
        (must_stmt
          arg: (must_expression))
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
        (container_stmt
          arg: (identifier))
        (list_stmt
          arg: (identifier))
        (leaf_stmt
          arg: (identifier)
          (type_stmt
            arg: (identifier)))
        (leaflist_stmt
          arg: (identifier)
          (type_stmt
            arg: (identifier)))
        (choice_stmt
          arg: (identifier))
        (anydata_stmt
          arg: (identifier))
        (anyxml_stmt
          arg: (identifier))
        (uses_stmt
          arg: (identifier))))))
        "#
    );
}
