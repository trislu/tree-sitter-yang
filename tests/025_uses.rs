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
    arg: (identifier)
    (container_stmt
      arg: (identifier)
      (uses_stmt
        arg: (identifier)
        (when_stmt
          arg: (string))
        (if_feature_stmt
          arg: (identifier))
        (status_stmt)
        (description
          arg: (qstring))
        (reference
          arg: (string))))))
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
    arg: (identifier)
    (container_stmt
      arg: (identifier)
      (uses_stmt
        arg: (identifier)
        (refine_stmt
          arg: (node_identifier
            (identifier))
          arg: (node_identifier
            (identifier))
          arg: (node_identifier
            (identifier))
          (if_feature_stmt
            arg: (identifier))
          (must_stmt
            arg: (must_expression))
          (presence_stmt
            arg: (string))
          (default_stmt
            arg: (integer_value))
          (mandatory_stmt)
          (min_elements_stmt)
          (max_elements_stmt)
          (description
            arg: (qstring))
          (reference
            arg: (string)))))))
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
    arg: (identifier)
    (container_stmt
      arg: (identifier)
      (uses_stmt
        arg: (identifier)
        (uses_augment_stmt
          arg: (node_identifier
            (identifier))
          arg: (node_identifier
            (identifier))
          arg: (node_identifier
            (identifier))
          (when_stmt
            arg: (string))
          (if_feature_stmt
            arg: (identifier))
          (status_stmt)
          (description
            arg: (qstring))
          (reference
            arg: (string))
          (container_stmt
            arg: (identifier))
          (list_stmt
            arg: (identifier))
          (leaf_stmt
            arg: (identifier)
            (type_stmt
              arg: (identifier)))
          (leaflist_stmt
            arg: (identifier)
            (type_stmt
              arg: (identifier)))
          (choice_stmt
            arg: (identifier))
          (anydata_stmt
            arg: (identifier))
          (anyxml_stmt
            arg: (identifier))
          (uses_stmt
            arg: (identifier))
          (case_stmt
            arg: (identifier)))))))
        "#
    );
}
