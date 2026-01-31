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
    arg: (identifier)))
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
    arg: (identifier)))
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
        https://www.rfc-editor.org/rfc/rfc9890";
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
    arg: (identifier)
    (yang_version)
    (prefix_stmt
      arg: (identifier))
    (namespace_stmt
      arg: (uri_str))
    (revision_stmt
      arg: (date_str))
    (import_stmt
      arg: (identifier)
      (prefix_stmt
        arg: (identifier))
      (revision_date
        arg: (date_str))
      (description
        arg: (string))
      (reference
        arg: (string)))
    (extension_stmt
      arg: (identifier)
      (description
        arg: (string))
      (argument_stmt
        arg: (identifier)
        (yin_element))
      (reference
        arg: (string))
      (status_stmt))))
"#
    );
}
