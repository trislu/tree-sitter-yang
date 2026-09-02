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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
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
              (identifier))))))))
        "#
    );
}
