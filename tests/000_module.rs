mod test_utils;

#[test]
fn test_empty_module() {
    parse_success_as!(
        r#"
module test {}
"#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))))
"#
    );
}

#[test]
fn test_module_name() {
    parse_success_as!(
        r#"
module "name-can-be-string" {}
"#,
        r#"
(yang
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))))
"#
    );
}

#[test]
fn test_module() {
    parse_success_as!(
        r#"
// module tree-sitter
module tree-sitter {
    yang-version 1.1;
    prefix ts;
    namespace "https://tree-sitter.github.io/tree-sitter/";
    revision 2026-01-31;
    import tree-sitter-yang {
        prefix ts-yang;
        revision-date 2026-01-31;
        description
        "Please see below YANG RFCs:
        https://www.rfc-editor.org/rfc/rfc6020
        https://www.rfc-editor.org/rfc/rfc7950 (version 1.1)
        https://www.rfc-editor.org/rfc/rfc9890"
          + '('
          + "maybe more"
          + ')';
        reference "https://github.com/trislu/tree-sitter-yang";
    }

    extension parser {
        description
        "The YANG language parser extension";
        argument "parse" {
            yin-element "true";
        }
        reference 'https://tree-sitter.github.io/tree-sitter/using-parsers/index.html';
        status current;
    }
}
    "#,
        r#"
(yang
  (comment)
  (module_stmt
    (module_keyword)
    arg: (module_arg_str
      (identifier))
    (yang_version_stmt
      (yang_version_keyword)
      arg: (yang_version_arg_str))
    (prefix_stmt
      (prefix_keyword)
      arg: (prefix_arg_str
        (identifier)))
    (namespace_stmt
      (namespace_keyword)
      arg: (namespace_arg_str))
    (revision_stmt
      (revision_keyword)
      arg: (revision_arg_str
        (date_str)))
    (import_stmt
      (import_keyword)
      arg: (import_arg_str
        (identifier))
      (prefix_stmt
        (prefix_keyword)
        arg: (prefix_arg_str
          (identifier)))
      (revision_date_stmt
        (revision_date_keyword)
        arg: (revision_date_arg_str
          (date_str)))
      (description_stmt
        (description_keyword)
        arg: (description_arg_str
          (quoted_string)
          (quoted_string)
          (quoted_string)
          (quoted_string)))
      (reference_stmt
        (reference_keyword)
        arg: (reference_arg_str
          (quoted_string))))
    (extension_stmt
      (extension_keyword)
      arg: (extension_arg_str
        (identifier))
      (description_stmt
        (description_keyword)
        arg: (description_arg_str
          (quoted_string)))
      (argument_stmt
        (argument_keyword)
        arg: (argument_arg_str
          (identifier))
        (yin_element_stmt
          (yin_element_keyword)
          arg: (yin_element_arg_str
            (boolean))))
      (reference_stmt
        (reference_keyword)
        arg: (reference_arg_str
          (quoted_string)))
      (status_stmt
        (status_keyword)
        arg: (status_arg_str)))))
"#
    );
}
