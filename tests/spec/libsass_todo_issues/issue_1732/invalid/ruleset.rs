//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1732/invalid/ruleset.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("color: green;\n"),
        "Error: expected \"{\".\
         \n  ,\
         \n1 | color: green;\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
