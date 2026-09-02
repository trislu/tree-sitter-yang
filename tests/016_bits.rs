mod test_utils;

#[test]
fn test_bit_basic() {
    parse_success_as!(
        r#"
module test{
    typedef mybits-type {
        type bits {
            bit disable-nagle {
                position 0;
            }
            bit auto-sense-speed {
                position 1;
            }
            bit ten-mb-only {
                position 2;
            }
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
    (typedef_stmt
      (typedef_keyword)
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        (type_keyword)
        arg: (type_arg_str
          (identifier))
        (bit_stmt
          (bit_keyword)
          arg: (bit_arg_str
            (identifier))
          (position_stmt
            (position_keyword)
            arg: (position_arg_str)))
        (bit_stmt
          (bit_keyword)
          arg: (bit_arg_str
            (identifier))
          (position_stmt
            (position_keyword)
            arg: (position_arg_str)))
        (bit_stmt
          (bit_keyword)
          arg: (bit_arg_str
            (identifier))
          (position_stmt
            (position_keyword)
            arg: (position_arg_str)))))))
        "#
    );
}
