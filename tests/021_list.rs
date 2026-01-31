mod test_utils;

#[test]
fn test_list() {
    parse_success_as!(
        r#"
module test{
    list foo {
        key "ip";
        leaf ip {type string;}
        leaf port {type uint16;}
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (list_stmt
      arg: (identifier)
      (key_stmt
        arg: (node_identifier
          (identifier)))
      (leaf_stmt
        arg: (identifier)
        (type_stmt
          arg: (identifier)))
      (leaf_stmt
        arg: (identifier)
        (type_stmt
          arg: (identifier))))))
        "#
    );
}

#[test]
fn test_nested_list() {
    parse_success_as!(
        r#"
module test{
    list foo {
        key ip;
        leaf ip {type string;}
        leaf port {type uint16;}
        list foo {
            key ip;
            leaf ip {type string;}
            leaf port {type uint16;}
        }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (list_stmt
      arg: (identifier)
      (key_stmt
        arg: (node_identifier
          (identifier)))
      (leaf_stmt
        arg: (identifier)
        (type_stmt
          arg: (identifier)))
      (leaf_stmt
        arg: (identifier)
        (type_stmt
          arg: (identifier)))
      (list_stmt
        arg: (identifier)
        (key_stmt
          arg: (node_identifier
            (identifier)))
        (leaf_stmt
          arg: (identifier)
          (type_stmt
            arg: (identifier)))
        (leaf_stmt
          arg: (identifier)
          (type_stmt
            arg: (identifier)))))))
        "#
    );
}

#[test]
fn test_list_full() {
    parse_success_as!(
        r#"
module test{
    list foo {
        when "what";
        if-feature xyz;
        must "be";
        key "ip port";
        unique "domain/user";
        config true;
        min-elements 2;
        max-elements 100;
        ordered-by user;
        status current;
        description "test full leaf";
        reference "tests/021_list.rs";
        leaf ip {type string;}
        leaf port {type uint16;}
        container domain {
            leaf user {type string;}
        }
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
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (list_stmt
      arg: (identifier)
      (when_stmt
        arg: (string))
      (if_feature_stmt
        arg: (identifier))
      (must_stmt
        arg: (must_expression))
      (key_stmt
        arg: (node_identifier
          (identifier))
        arg: (node_identifier
          (identifier)))
      (unique_stmt
        arg: (node_identifier
          (identifier))
        arg: (node_identifier
          (identifier)))
      (config_stmt)
      (min_elements_stmt)
      (max_elements_stmt)
      (ordered_by_stmt)
      (status_stmt)
      (description
        arg: (string))
      (reference
        arg: (string))
      (leaf_stmt
        arg: (identifier)
        (type_stmt
          arg: (identifier)))
      (leaf_stmt
        arg: (identifier)
        (type_stmt
          arg: (identifier)))
      (container_stmt
        arg: (identifier)
        (leaf_stmt
          arg: (identifier)
          (type_stmt
            arg: (identifier))))
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
