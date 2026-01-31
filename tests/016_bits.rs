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
    arg: (identifier)
    (typedef_stmt
      arg: (identifier)
      (type_stmt
        arg: (identifier)
        (bit_stmt
          arg: (identifier)
          (position_stmt))
        (bit_stmt
          arg: (identifier)
          (position_stmt))
        (bit_stmt
          arg: (identifier)
          (position_stmt))))))
        "#
    );
}
