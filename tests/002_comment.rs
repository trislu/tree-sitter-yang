mod test_utils;

#[test]
fn test_line_comment() {
    parse_success_as!(
        r#"
// line comment 1
module empty {
    // line comment 2
}
// line comment 3
"#,
        r#"
(yang
  (comment)
  (module
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
module /*2*/  empty /*3*/ {
    /*4*/ 
}
/*5*/
"#,
        r#"
(yang
  (comment)
  (module
    (comment)
    arg: (identifier)
    (comment)
    (comment))
  (comment))
"#
    );
}
