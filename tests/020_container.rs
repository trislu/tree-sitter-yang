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
    arg: (module_arg_str
      (identifier))
    (container_stmt
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
    arg: (module_arg_str
      (identifier))
    (container_stmt
      arg: (container_arg_str
        (identifier))
      (container_stmt
        arg: (container_arg_str
          (identifier))
        (container_stmt
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
    arg: (module_arg_str
      (identifier))
    (container_stmt
      arg: (container_arg_str
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
              (identifier)))))
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
            (prefix
              (identifier))
            (identifier)))))))
        "#
    );
}
