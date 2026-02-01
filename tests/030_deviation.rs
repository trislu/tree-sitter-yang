mod test_utils;

#[test]
fn test_deviation() {
    parse_success_as!(
        r#"
module test{
    deviation /foo/bar {
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (deviation_stmt
      arg: (node_identifier
        (identifier))
      arg: (node_identifier
        (identifier)))))
        "#
    );
}

#[test]
fn test_deviation_not_supported() {
    parse_success_as!(
        r#"
module test{
    deviation /base:system/base:daytime {
      deviate not-supported;
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (deviation_stmt
      arg: (node_identifier
        (identifier)
        (identifier))
      arg: (node_identifier
        (identifier)
        (identifier))
      (deviate_not_supported_stmt))))
        "#
    );
}

#[test]
fn test_deviation_add() {
    parse_success_as!(
        r#"
module test{
  deviation /base:system/base:user/base:type {
    deviate add {
      default "admin"; // new users are 'admin' by default
    }
  }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (deviation_stmt
      arg: (node_identifier
        (identifier)
        (identifier))
      arg: (node_identifier
        (identifier)
        (identifier))
      arg: (node_identifier
        (identifier)
        (identifier))
      (deviate_add_stmt
        (default_stmt
          arg: (string))
        (comment)))))
        "#
    );
}

#[test]
fn test_deviation_replace() {
    parse_success_as!(
        r#"
module test{
    deviation /base:system/base:name-server {
      deviate replace {
        max-elements 3;
      }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (deviation_stmt
      arg: (node_identifier
        (identifier)
        (identifier))
      arg: (node_identifier
        (identifier)
        (identifier))
      (deviate_replace_stmt
        (max_elements_stmt)))))
        "#
    );
}

#[test]
fn test_deviation_delete() {
    parse_success_as!(
        r#"
module test{
    deviation /base:system {
      deviate delete {
        must "daytime or time";
      }
    }
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (deviation_stmt
      arg: (node_identifier
        (identifier)
        (identifier))
      (deviate_delete_stmt
        (must_stmt
          arg: (must_expression))))))
        "#
    );
}
