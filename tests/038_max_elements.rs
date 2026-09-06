mod test_utils;

// Regression: `max-elements unbounded;` used to collapse the whole file
// (the value rule only accepted a positive integer). RFC 7950 §14:
// max-value-arg = unbounded-keyword / positive-integer-value.

#[test]
fn max_elements_unbounded_parses() {
    parse_success_as!(
        r#"
module m {
  yang-version 1.1;
  namespace "urn:x";
  prefix x;
  list l {
    key "k";
    max-elements unbounded;
    leaf k { type string; }
  }
}
"#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (yang_version_stmt
      (yang_version_keyword)
      arg: (yang_version_arg_str))
    (namespace_stmt
      (namespace_keyword)
      arg: (namespace_arg_str))
    (prefix_stmt
      (prefix_keyword)
      arg: (prefix_arg_str
        (identifier)))
    (list_stmt
      (list_keyword)
      arg: (list_arg_str
        (identifier))
      (key_stmt
        (key_keyword)
        arg: (key_arg_str
          (string
            (quoted_string))))
      (max_elements_stmt
        (max_elements_keyword))
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
fn max_elements_unbounded_quoted_parses() {
    parse_success_as!(
        r#"
module m {
  namespace "urn:x";
  prefix x;
  list l {
    key k;
    max-elements "unbounded";
    leaf k { type string; }
  }
}
"#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (namespace_stmt
      (namespace_keyword)
      arg: (namespace_arg_str))
    (prefix_stmt
      (prefix_keyword)
      arg: (prefix_arg_str
        (identifier)))
    (list_stmt
      (list_keyword)
      arg: (list_arg_str
        (identifier))
      (key_stmt
        (key_keyword)
        arg: (key_arg_str
          (string
            (identifier))))
      (max_elements_stmt
        (max_elements_keyword))
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
