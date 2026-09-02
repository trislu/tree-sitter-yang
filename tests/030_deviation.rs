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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (deviation_stmt
      (deviation_keyword)
      arg: (deviation_arg_str
        (node_identifier
          (identifier))
        (node_identifier
          (identifier))))))
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (deviation_stmt
      (deviation_keyword)
      arg: (deviation_arg_str
        (node_identifier
          (identifier)
          (identifier))
        (node_identifier
          (identifier)
          (identifier)))
      (deviate_not_supported_stmt
        (deviate_keyword)
        arg: (not_supported_arg_str)))))
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (deviation_stmt
      (deviation_keyword)
      arg: (deviation_arg_str
        (node_identifier
          (identifier)
          (identifier))
        (node_identifier
          (identifier)
          (identifier))
        (node_identifier
          (identifier)
          (identifier)))
      (deviate_add_stmt
        (deviate_keyword)
        arg: (add_arg_str)
        (default_stmt
          (default_keyword)
          arg: (default_arg_str
            (quoted_string)))
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (deviation_stmt
      (deviation_keyword)
      arg: (deviation_arg_str
        (node_identifier
          (identifier)
          (identifier))
        (node_identifier
          (identifier)
          (identifier)))
      (deviate_replace_stmt
        (deviate_keyword)
        arg: (replace_arg_str)
        (max_elements_stmt
          (max_elements_keyword))))))
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
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (deviation_stmt
      (deviation_keyword)
      arg: (deviation_arg_str
        (node_identifier
          (identifier)
          (identifier)))
      (deviate_delete_stmt
        (deviate_keyword)
        arg: (delete_arg_str)
        (must_stmt
          (must_keyword)
          arg: (must_expression
            (quoted_string)))))))
        "#
    );
}
