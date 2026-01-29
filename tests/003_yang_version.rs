mod test_utils;

#[test]
fn test_yang_version_number() {
    parse_success_as!(
        r#"
module empty {
    yang-version 1;
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (yang_version)))
"#
    );

    parse_success_as!(
        r#"
module empty {
    yang-version 1.1;
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (yang_version)))
"#
    );
}

#[test]
fn test_yang_version_string() {
    parse_success_as!(
        r#"
module empty {
    yang-version "1";
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (yang_version)))
"#
    );

    parse_success_as!(
        r#"
module empty {
    yang-version '1.1';
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (yang_version)))
"#
    );
}

#[test]
fn test_yang_version_0() {
    parse_error_as!(
        r#"    
module empty {
    yang-version 0;
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (ERROR
      (UNEXPECTED '0'))))
        "#
    );
}

#[test]
fn test_yang_version_2() {
    parse_error_as!(
        r#"    
module empty {
    yang-version 2;
}
    "#,
        r#"
(yang
  (module
    arg: (identifier)
    (ERROR
      (UNEXPECTED '2'))))
        "#
    );
}
