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
    arg: (identifier)
    (container_stmt
      arg: (identifier))))
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
    arg: (identifier)
    (container_stmt
      arg: (identifier)
      (container_stmt
        arg: (identifier)
        (container_stmt
          arg: (identifier))))))
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
    arg: (identifier)
    (container_stmt
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
            arg: (identifier))))
      (leaf_stmt
        arg: (identifier)
        (type_stmt
          arg: (identifier)))
      (leaflist_stmt
        arg: (identifier)
        (type_stmt
          arg: (prefix
            (identifier))
          arg: (identifier))))))
        "#
    );
}
