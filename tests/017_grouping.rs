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
    arg: (module_arg_str
      (identifier))
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
            (identifier)))))))
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
    arg: (module_arg_str
      (identifier))
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
              (identifier))))))))
        "#
    );
}
