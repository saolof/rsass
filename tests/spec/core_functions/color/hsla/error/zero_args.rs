//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/error/zero_args.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
             \n  b: hsla();\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Missing argument $channels.\
         \n  ,--> input.scss\
         \n2 |   b: hsla();\
         \n  |      ^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function hsla($channels) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}