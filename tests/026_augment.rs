mod test_utils;

#[test]
fn test_augment() {
    parse_success_as!(
        r#"
module test{
    augment "/b:x" {
        if-feature foo;
        leaf y {
            type myenum;
        }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (augment_stmt
      arg: (node_identifier
        (identifier)
        (identifier))
      (if_feature_stmt
        arg: (identifier))
      (leaf_stmt
        arg: (identifier)
        (type_stmt
          arg: (identifier))))))
        "#
    );
}

#[test]
fn test_augment_full() {
    parse_success_as!(
        r#"
module test{
    augment "/b:x" {
        when "xyz";
        if-feature foo;
        status deprecated;
        description "test";
        reference "test_augment_full.com";
        leaf y {
            type myenum;
        }
        case bar {}
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (augment_stmt
      arg: (node_identifier
        (identifier)
        (identifier))
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
      (case_stmt
        arg: (identifier)))))
        "#
    );
}
