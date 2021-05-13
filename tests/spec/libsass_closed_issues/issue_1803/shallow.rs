//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1803/shallow.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  display: block\n\
             \n  b {\
             \n    foo: bar;\
             \n  }\
             \n}\n"),
        "a {\
         \n  display: block b;\
         \n  display-foo: bar;\
         \n}\n"
    );
}
