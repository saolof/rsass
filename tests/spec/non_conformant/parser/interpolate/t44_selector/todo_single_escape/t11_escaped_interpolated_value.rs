//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/todo_single_escape/11_escaped_interpolated_value.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "$key: \'bar\';\
             \n.test11#{\'\\@#{$key}\'} { content: \'1.1\'; }\n"
        ),
        "Error: expected selector.\
         \n  ,\
         \n2 | .test11@bar{ content: \'1.1\'; }\
         \n  |        ^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet",
    );
}
