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
    arg: (module_arg_str
      (identifier))
    (typedef_stmt
      arg: (typedef_arg_str
        (identifier))
      (type_stmt
        arg: (type_arg_str
          (identifier))
        (bit_stmt
          arg: (bit_arg_str
            (identifier))
          (position_stmt
            arg: (position_arg_str)))
        (bit_stmt
          arg: (bit_arg_str
            (identifier))
          (position_stmt
            arg: (position_arg_str)))
        (bit_stmt
          arg: (bit_arg_str
            (identifier))
          (position_stmt
            arg: (position_arg_str)))))))
        "#
    );
}
