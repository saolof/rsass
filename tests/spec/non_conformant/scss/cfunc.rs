//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/cfunc.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  blah: say-something();\
             \n}"),
        "div {\
         \n  blah: say-something();\
         \n}\n"
    );
}
