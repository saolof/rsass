//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_575.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".test {\
             \n  @if (foo: bar) == (foo: bar) {\
             \n    foo: bar;\
             \n  }\
             \n}\n"),
        ".test {\
         \n  foo: bar;\
         \n}\n"
    );
}
