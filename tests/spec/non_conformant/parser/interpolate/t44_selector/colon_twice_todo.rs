//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/colon_twice_todo.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "test {\
             \n  filter: progid:DXImageTransform.Microsoft.#{Alpha(opacity=20)};\
             \n}\n"
        ),
        "Error: expected \"(\".\
         \n  ,\
         \n2 |   filter: progid:DXImageTransform.Microsoft.#{Alpha(opacity=20)};\
         \n  |                                             ^\
         \n  \'\
         \n  input.scss 2:45  root stylesheet",
    );
}
