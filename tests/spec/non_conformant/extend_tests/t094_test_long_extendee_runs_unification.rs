//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/094_test_long_extendee_runs_unification.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "ns|*.foo.bar {a: b}\
             \na.baz {@extend .foo.bar}\
             \n"
        )
        .unwrap_err(),
        "Error: compound selectors may no longer be extended.\
         \nConsider `@extend .foo, .bar` instead.\
         \nSee http://bit.ly/ExtendCompound for details.\
         \n\
         \n  ,\
         \n2 | a.baz {@extend .foo.bar}\
         \n  |                ^^^^^^^^\
         \n  \'\
         \n  input.scss 2:16  root stylesheet",
    );
}