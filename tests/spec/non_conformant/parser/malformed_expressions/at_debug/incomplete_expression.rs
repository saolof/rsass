//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/malformed_expressions/at-debug/incomplete-expression.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@debug(\"foo\";\n"),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @debug(\"foo\";\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
