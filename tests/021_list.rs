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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (list_stmt
      (list_keyword)
      arg: (list_arg_str
        (identifier))
      (key_stmt
        (key_keyword)
        arg: (key_arg_str
          (string
            (quoted_string))))
      (leaf_stmt
        (leaf_keyword)
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          (type_keyword)
          arg: (type_arg_str
            (identifier))))
      (leaf_stmt
        (leaf_keyword)
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          (type_keyword)
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (list_stmt
      (list_keyword)
      arg: (list_arg_str
        (identifier))
      (key_stmt
        (key_keyword)
        arg: (key_arg_str
          (string
            (identifier))))
      (leaf_stmt
        (leaf_keyword)
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          (type_keyword)
          arg: (type_arg_str
            (identifier))))
      (leaf_stmt
        (leaf_keyword)
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          (type_keyword)
          arg: (type_arg_str
            (identifier))))
      (list_stmt
        (list_keyword)
        arg: (list_arg_str
          (identifier))
        (key_stmt
          (key_keyword)
          arg: (key_arg_str
            (string
              (identifier))))
        (leaf_stmt
          (leaf_keyword)
          arg: (leaf_arg_str
            (identifier))
          (type_stmt
            (type_keyword)
            arg: (type_arg_str
              (identifier))))
        (leaf_stmt
          (leaf_keyword)
          arg: (leaf_arg_str
            (identifier))
          (type_stmt
            (type_keyword)
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (list_stmt
      (list_keyword)
      arg: (list_arg_str
        (identifier))
      (when_stmt
        (when_keyword)
        arg: (string
          (quoted_string)))
      (if_feature_stmt
        (if_feature_keyword)
        arg: (if_feature_arg_str
          (identifier)))
      (must_stmt
        (must_keyword)
        arg: (must_expression
          (quoted_string)))
      (key_stmt
        (key_keyword)
        arg: (key_arg_str
          (string
            (quoted_string))))
      (unique_stmt
        (unique_keyword)
        arg: (unique_arg_str
          (string
            (quoted_string))))
      (config_stmt
        (config_keyword)
        arg: (boolean))
      (min_elements_stmt
        (min_elements_keyword))
      (max_elements_stmt
        (max_elements_keyword))
      (ordered_by_stmt
        (ordered_by_keyword))
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
      (leaf_stmt
        (leaf_keyword)
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          (type_keyword)
          arg: (type_arg_str
            (identifier))))
      (leaf_stmt
        (leaf_keyword)
        arg: (leaf_arg_str
          (identifier))
        (type_stmt
          (type_keyword)
          arg: (type_arg_str
            (identifier))))
      (container_stmt
        (container_keyword)
        arg: (container_arg_str
          (identifier))
        (leaf_stmt
          (leaf_keyword)
          arg: (leaf_arg_str
            (identifier))
          (type_stmt
            (type_keyword)
            arg: (type_arg_str
              (identifier)))))
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
