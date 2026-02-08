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
    arg: (module_arg_str
      (identifier))
    (list_stmt
      arg: (list_arg_str
        (identifier))
      (key_stmt
        arg: (key_arg_str
          (node_identifier
            (identifier))))
      (leaf_stmt
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          arg: (type_arg_str
            (identifier))))
      (leaf_stmt
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          arg: (type_arg_str
            (identifier)))))))
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
    arg: (module_arg_str
      (identifier))
    (list_stmt
      arg: (list_arg_str
        (identifier))
      (key_stmt
        arg: (key_arg_str
          (node_identifier
            (identifier))))
      (leaf_stmt
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          arg: (type_arg_str
            (identifier))))
      (leaf_stmt
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          arg: (type_arg_str
            (identifier))))
      (list_stmt
        arg: (list_arg_str
          (identifier))
        (key_stmt
          arg: (key_arg_str
            (node_identifier
              (identifier))))
        (leaf_stmt
          arg: (leaf_arg_str
            (identifier))
          (type_stmt
            arg: (type_arg_str
              (identifier))))
        (leaf_stmt
          arg: (leaf_arg_str
            (identifier))
          (type_stmt
            arg: (type_arg_str
              (identifier))))))))
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
    arg: (module_arg_str
      (identifier))
    (list_stmt
      arg: (list_arg_str
        (identifier))
      (when_stmt
        arg: (string))
      (if_feature_stmt
        arg: (if_feature_arg_str
          (identifier)))
      (must_stmt
        arg: (must_expression))
      (key_stmt
        arg: (key_arg_str
          (node_identifier
            (identifier))
          (node_identifier
            (identifier))))
      (unique_stmt
        arg: (unique_arg_str
          (node_identifier
            (identifier))
          (node_identifier
            (identifier))))
      (config_stmt
        arg: (boolean))
      (min_elements_stmt)
      (max_elements_stmt)
      (ordered_by_stmt)
      (status_stmt
        arg: (status_arg_str))
      (description_stmt
        arg: (description_arg_str))
      (reference_stmt
        arg: (reference_arg_str))
      (leaf_stmt
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          arg: (type_arg_str
            (identifier))))
      (leaf_stmt
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          arg: (type_arg_str
            (identifier))))
      (container_stmt
        arg: (container_arg_str
          (identifier))
        (leaf_stmt
          arg: (leaf_arg_str
            (identifier))
          (type_stmt
            arg: (type_arg_str
              (identifier)))))
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
