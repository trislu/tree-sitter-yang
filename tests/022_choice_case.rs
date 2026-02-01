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
    arg: (identifier)
    (choice_stmt
      arg: (identifier))))
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
    arg: (identifier)
    (choice_stmt
      arg: (identifier)
      (choice_stmt
        arg: (identifier)
        (choice_stmt
          arg: (identifier))))))
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
    arg: (identifier)
    (choice_stmt
      arg: (identifier)
      (when_stmt
        arg: (string))
      (if_feature_stmt
        arg: (identifier))
      (status_stmt)
      (description_stmt
        arg: (qstring))
      (reference_stmt
        arg: (string))
      (leaf_stmt
        arg: (identifier)
        (type_stmt
          arg: (identifier)))
      (leaf_list_stmt
        arg: (identifier)
        (type_stmt
          arg: (prefix
            (identifier))
          arg: (identifier)))
      (case_stmt
        arg: (identifier)
        (when_stmt
          arg: (string))
        (if_feature_stmt
          arg: (identifier))
        (status_stmt)
        (description_stmt
          arg: (qstring))
        (reference_stmt
          arg: (string))
        (container_stmt
          arg: (identifier))
        (list_stmt
          arg: (identifier))
        (leaf_stmt
          arg: (identifier)
          (type_stmt
            arg: (identifier)))
        (leaf_list_stmt
          arg: (identifier)
          (type_stmt
            arg: (identifier)))))))
        "#
    );
}
