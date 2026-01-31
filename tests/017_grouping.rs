mod test_utils;

#[test]
fn test_grouping_basic() {
    parse_success_as!(
        r#"
module test{
    grouping my-group {
        status obsolete;
        description
        "my-group is just my group";
        reference "http://my-group";
        typedef my-type {
            type uint32;
        }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (grouping_stmt
      arg: (identifier)
      (status_stmt)
      (description
        arg: (string))
      (reference
        arg: (string))
      (typedef_stmt
        arg: (identifier)
        (type_stmt
          arg: (identifier))))))
        "#
    );
}

#[test]
fn test_grouping_nested() {
    parse_success_as!(
        r#"
module test{
    grouping my-group {
        status obsolete;
        description
        "my-group is just my group";
        reference "http://my-group";
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
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (grouping_stmt
      arg: (identifier)
      (status_stmt)
      (description
        arg: (string))
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
          arg: (string))
        (reference
          arg: (string))
        (typedef_stmt
          arg: (identifier)
          (type_stmt
            arg: (identifier)))))))
        "#
    );
}
