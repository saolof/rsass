//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/12_escaped_double_quoted/04_variable_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"l\\\\ite\\ral\";\
             \n.result {\
             \n  output: #{#{$input}};\
             \n  output: #{\"[#{$input}]\"};\
             \n  output: #{\"#{$input}\"};\
             \n  output: #{\'#{$input}\'};\
             \n  output: #{\"[\'#{$input}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: l\\iteral;\
         \n  output: [l\\iteral];\
         \n  output: l\\iteral;\
         \n  output: l\\iteral;\
         \n  output: [\'l\\iteral\'];\
         \n}\n"
    );
}
