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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (container_stmt
      (container_keyword)
      arg: (container_arg_str
        (identifier))
      (uses_stmt
        (uses_keyword)
        arg: (uses_arg_str
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
            (quoted_string)))))))
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (container_stmt
      (container_keyword)
      arg: (container_arg_str
        (identifier))
      (uses_stmt
        (uses_keyword)
        arg: (uses_arg_str
          (identifier))
        (refine_stmt
          (refine_keyword)
          arg: (refine_arg_str
            (node_identifier
              (identifier))
            (node_identifier
              (identifier))
            (node_identifier
              (identifier)))
          (if_feature_stmt
            (if_feature_keyword)
            arg: (if_feature_arg_str
              (identifier)))
          (must_stmt
            (must_keyword)
            arg: (must_expression
              (quoted_string)))
          (presence_stmt
            (presence_keyword)
            arg: (string
              (quoted_string)))
          (default_stmt
            (default_keyword)
            arg: (default_arg_str))
          (mandatory_stmt
            (mandatory_keyword)
            arg: (boolean))
          (min_elements_stmt
            (min_elements_keyword))
          (max_elements_stmt
            (max_elements_keyword))
          (description_stmt
            (description_keyword)
            arg: (description_arg_str
              (quoted_string)))
          (reference_stmt
            (reference_keyword)
            arg: (reference_arg_str
              (quoted_string))))))))
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (container_stmt
      (container_keyword)
      arg: (container_arg_str
        (identifier))
      (uses_stmt
        (uses_keyword)
        arg: (uses_arg_str
          (identifier))
        (uses_augment_stmt
          (augment_keyword)
          arg: (uses_augment_arg_str
            (node_identifier
              (identifier))
            (node_identifier
              (identifier))
            (node_identifier
              (identifier)))
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
                (identifier))))
          (choice_stmt
            (choice_keyword)
            arg: (choice_arg_str
              (identifier)))
          (anydata_stmt
            (anydata_keyword)
            arg: (anydata_arg_str
              (identifier)))
          (anyxml_stmt
            (anyxml_keyword)
            arg: (anyxml_arg_str
              (identifier)))
          (uses_stmt
            (uses_keyword)
            arg: (uses_arg_str
              (identifier)))
          (case_stmt
            (case_keyword)
            arg: (case_arg_str
              (identifier))))))))
        "#
    );
}
