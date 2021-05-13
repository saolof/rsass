//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/13_escaped_single_quoted/06_escape_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \'l\\\\ite\\ral\';\
             \n.result {\
             \n  output: \"[\\#{\'l\\\\ite\\ral\'}]\";\
             \n  output: \"\\#{\'l\\\\ite\\ral\'}\";\
             \n  output: \'\\#{\'l\\\\ite\\ral\'}\';\
             \n  output: \"[\'\\#{\'l\\\\ite\\ral\'}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[#{\'l\\\\iteral\'}]\";\
         \n  output: \"#{\'l\\\\iteral\'}\";\
         \n  output: \"#{\" l\\\\iteral \"}\";\
         \n  output: \"[\'#{\'l\\\\iteral\'}\']\";\
         \n}\n"
    );
}
