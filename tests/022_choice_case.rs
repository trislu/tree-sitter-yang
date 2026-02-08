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
    arg: (module_arg_str
      (identifier))
    (choice_stmt
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
    arg: (module_arg_str
      (identifier))
    (choice_stmt
      arg: (choice_arg_str
        (identifier))
      (choice_stmt
        arg: (choice_arg_str
          (identifier))
        (choice_stmt
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
    arg: (module_arg_str
      (identifier))
    (choice_stmt
      arg: (choice_arg_str
        (identifier))
      (when_stmt
        arg: (string))
      (if_feature_stmt
        arg: (if_feature_arg_str
          (identifier)))
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
      (leaf_list_stmt
        arg: (leaf_list_arg_str
          (identifier))
        (type_stmt
          arg: (type_arg_str
            (prefix
              (identifier))
            (identifier))))
      (case_stmt
        arg: (case_arg_str
          (identifier))
        (when_stmt
          arg: (string))
        (if_feature_stmt
          arg: (if_feature_arg_str
            (identifier)))
        (status_stmt
          arg: (status_arg_str))
        (description_stmt
          arg: (description_arg_str))
        (reference_stmt
          arg: (reference_arg_str))
        (container_stmt
          arg: (container_arg_str
            (identifier)))
        (list_stmt
          arg: (list_arg_str
            (identifier)))
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
              (identifier))))))))
        "#
    );
}
