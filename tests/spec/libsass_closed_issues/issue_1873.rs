//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1873.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a {\
             \n  display: block;\
             \n}\
             \n\
             \n.b {\
             \n  @at-root (with: media) {\
             \n    @extend .a;\
             \n  }\
             \n}"
        )
        .unwrap_err(),
        "Error: @extend may only be used within style rules.\
         \n  ,\
         \n7 |     @extend .a;\
         \n  |     ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 7:5  root stylesheet\
         \n",
    );
}
