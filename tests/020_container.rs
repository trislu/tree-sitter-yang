mod test_utils;

#[test]
fn test_container() {
    parse_success_as!(
        r#"
module test{
    container foo {
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
        (identifier)))))
        "#
    );
}

#[test]
fn test_nested_container() {
    parse_success_as!(
        r#"
module test{
    container foo {
        container bar {
            container baz {
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
      (container_stmt
        (container_keyword)
        arg: (container_arg_str
          (identifier))
        (container_stmt
          (container_keyword)
          arg: (container_arg_str
            (identifier)))))))
        "#
    );
}

#[test]
fn test_container_full() {
    parse_success_as!(
        r#"
module test{
    container foo {
        status obsolete;
        description
        "foo is just foo";
        reference "http://foo";
        typedef foo-type {
            type uint32;
        }
        grouping foo-group {
            status current;
            description
            "whatever";
            reference "http://foo/foo-group";
            typedef foo-group-type {
                type string;
            }
        }
        leaf bar {
            type foo-type;
        }
        leaf-list baz {
            type foo-group:foo-group-type;
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
            (prefix
              (identifier))
            (identifier)))))))
        "#
    );
}
