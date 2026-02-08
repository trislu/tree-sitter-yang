mod test_utils;

#[test]
fn test_uses() {
    parse_success_as!(
        r#"
module test{
    container data {
        uses foo {
            when "what";
            if-feature xyz;
            status current;
            description "test uses";
            reference "tests/025_uses.rs";
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
      (uses_stmt
        arg: (uses_arg_str
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
          arg: (reference_arg_str))))))
        "#
    );
}

#[test]
fn test_uses_refine() {
    parse_success_as!(
        r#"
module test{
    container data {
        uses foo {
            refine "x/y/z" {
                if-feature xyz;
                must "xyz";
                presence "xyz";
                default 100;
                mandatory false;
                min-elements 123;
                max-elements 456{}
                description "test uses";
                reference "tests/025_uses.rs";
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
      (uses_stmt
        arg: (uses_arg_str
          (identifier))
        (refine_stmt
          arg: (refine_arg_str
            (node_identifier
              (identifier))
            (node_identifier
              (identifier))
            (node_identifier
              (identifier)))
          (if_feature_stmt
            arg: (if_feature_arg_str
              (identifier)))
          (must_stmt
            arg: (must_expression))
          (presence_stmt
            arg: (string))
          (default_stmt
            arg: (default_arg_str))
          (mandatory_stmt
            arg: (boolean))
          (min_elements_stmt)
          (max_elements_stmt)
          (description_stmt
            arg: (description_arg_str))
          (reference_stmt
            arg: (reference_arg_str)))))))
        "#
    );
}

#[test]
fn test_uses_augment() {
    parse_success_as!(
        r#"
module test{
    container data {
        uses foo {
            augment "x/y/z" {
                when "xyz";
                if-feature xyz;
                status obsolete;
                description "test uses";
                reference "tests/025_uses.rs";
                container foo{}
                list bar {}
                leaf baz {type string;}
                leaf-list qux {type string;}
                choice what {}
                anydata data;
                anyxml xml;
                uses bar;
                case xyz {}
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
      (uses_stmt
        arg: (uses_arg_str
          (identifier))
        (uses_augment_stmt
          arg: (uses_augment_arg_str
            (node_identifier
              (identifier))
            (node_identifier
              (identifier))
            (node_identifier
              (identifier)))
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
                (identifier))))
          (choice_stmt
            arg: (choice_arg_str
              (identifier)))
          (anydata_stmt
            arg: (anydata_arg_str
              (identifier)))
          (anyxml_stmt
            arg: (anyxml_arg_str
              (identifier)))
          (uses_stmt
            arg: (uses_arg_str
              (identifier)))
          (case_stmt
            arg: (case_arg_str
              (identifier))))))))
        "#
    );
}
