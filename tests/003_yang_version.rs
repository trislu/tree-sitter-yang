mod test_utils;

#[test]
fn test_yang_version_number() {
    parse_success_as!(
        r#"
module test {
    yang-version 1;
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (yang_version_stmt)))
"#
    );
    parse_success_as!(
        r#"
module test {
    yang-version 1.1;
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (yang_version_stmt)))
"#
    );
}

#[test]
fn test_yang_version_string() {
    parse_success_as!(
        r#"
module test {
    yang-version "1";
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (yang_version_stmt)))
"#
    );
    parse_success_as!(
        r#"
module test {
    yang-version '1.1';
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (yang_version_stmt)))
"#
    );
}

#[test]
#[ignore = "this test case is very unstable, so bizarre."]
fn test_invalid_yang_version() {
    // okay, 0 is unexpected
    parse_error_as!(
        r#"
module test {
    yang-version 0;
}
    "#,
        r#"
(yang
  (module_stmt
    arg: (identifier)
    (ERROR
      (UNEXPECTED '0'))))
        "#
    );
    // bigger numbers seems to be "consumed"
    for i in 2..9 {
        let code = format!(r#"module test {{ yang-version {}; }}"#, i);
        parse_error_as!(
            &code,
            r#"
(yang
  (module_stmt
    arg: (identifier)
    (ERROR
      (UNEXPECTED ';'))))
        "#
        );
    }
}

#[test]
#[ignore = "this test case is very unstable, so bizarre."]
fn test_range_a_to_z_upper() {
    // upper [A-Z] are reasonable as 0
    for c in 'A'..='Z' {
        let code = format!(r#"module test {{ yang-version {}; }}"#, c);
        let expected_error_sexp = format!(
            r#"
(yang
  (module_stmt
    arg: (identifier)
    (ERROR
      (UNEXPECTED '{}'))))
        "#,
            c
        );
        parse_error_as!(&code, expected_error_sexp);
    }
}

#[test]
#[ignore = "this test case is very unstable, so bizarre."]
fn test_range_a_to_z_lower() {
    for c in 'a'..='z' {
        let code = format!(r#"module test {{ yang-version {}; }}"#, c);
        let expected_error_sexp = format!(
            r#"
(yang
  (module_stmt
    arg: (identifier)
    (ERROR
      (UNEXPECTED '{}'))))
        "#,
            match c {// seem to be the first character of all the define rules
                'b'| // b'elongs-to
                'c'| // c'ontact
                'd'| // d'escription
                'm'| // m'odule
                'n'| // n'amespace
                'o'| // o'rganization
                'p'| // p'refix
                'r'| // r'evision
                's'| // s'ubmodule
                'y'  // y'ang-version
                    => ';',
                _ => c,
            }
        );
        parse_error_as!(&code, expected_error_sexp);
    }
}
