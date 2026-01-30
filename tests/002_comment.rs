mod test_utils;

#[test]
fn test_line_comment() {
    parse_success_as!(
        r#"
// line comment 1
module test {
    // line comment 2
}
// line comment 3
"#,
        r#"
(yang
  (comment)
  (module_stmt
    arg: (identifier)
    (comment))
  (comment))
"#
    );
}

#[test]
fn test_block_comment() {
    parse_success_as!(
        r#"        
/*1*/ 
module /*2*/  test /*3*/ {
    /*4*/ 
}
/*5*/
"#,
        r#"
(yang
  (comment)
  (module_stmt
    (comment)
    arg: (identifier)
    (comment)
    (comment))
  (comment))
"#
    );
}
