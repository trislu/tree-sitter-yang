mod test_utils;

#[test]
fn test_choice() {
    parse_success_as!(
        r#"
module test{
    choice foo {
    }
}
    "#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (choice_stmt
      (choice_keyword)
      arg: (choice_arg_str
        (identifier)))))
        "#
    );
}

#[test]
fn test_nested_choice() {
    parse_success_as!(
        r#"
module test{
    choice foo {
        choice bar {
            choice baz {
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
    (choice_stmt
      (choice_keyword)
      arg: (choice_arg_str
        (identifier))
      (choice_stmt
        (choice_keyword)
        arg: (choice_arg_str
          (identifier))
        (choice_stmt
          (choice_keyword)
          arg: (choice_arg_str
            (identifier)))))))
        "#
    );
}

#[test]
fn test_choice_case_full() {
    parse_success_as!(
        r#"
module test{
    choice foo {
        when "zzz";
        if-feature xyz;
        status obsolete;
        description "foo is just foo";
        reference "http://foo";
        leaf bar {
            type foo-type;
        }
        leaf-list baz {
            type foo-group:foo-group-type;
        }
        case qux {
            when "qqq";
            if-feature xyz;
            status obsolete;
            description "qux is just qux";
            reference "http://qux";
            container x {}
            list y {}
            leaf z {type uint32;}
            leaf-list a {type string;}
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
    (choice_stmt
      (choice_keyword)
      arg: (choice_arg_str
        (identifier))
      (when_stmt
        (when_keyword)
        arg: (string
          (quoted_string)))
      (if_feature_stmt
        (if_feature_keyword)
        arg: (if_feature_arg_str
          (identifier)))
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
      (leaf_list_stmt
        (leaf_list_keyword)
        arg: (leaf_list_arg_str
          (identifier))
        (type_stmt
          (type_keyword)
          arg: (type_arg_str
            (prefix
              (identifier))
            (identifier))))
      (case_stmt
        (case_keyword)
        arg: (case_arg_str
          (identifier))
        (when_stmt
          (when_keyword)
          arg: (string
            (quoted_string)))
        (if_feature_stmt
          (if_feature_keyword)
          arg: (if_feature_arg_str
            (identifier)))
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
        (container_stmt
          (container_keyword)
          arg: (container_arg_str
            (identifier)))
        (list_stmt
          (list_keyword)
          arg: (list_arg_str
            (identifier)))
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
              (identifier))))))))
        "#
    );
}
