//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/30_base_test/06_escape_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"foo#{\'ba\' + \'r\'}baz\";\
             \n.result {\
             \n  output: \"[\\#{\"foo#{\'ba\' + \'r\'}baz\"}]\";\
             \n  output: \"\\#{\"foo#{\'ba\' + \'r\'}baz\"}\";\
             \n  output: \'\\#{\"foo#{\'ba\' + \'r\'}baz\"}\';\
             \n  output: \"[\'\\#{\"foo#{\'ba\' + \'r\'}baz\"}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[#{\" foobarbaz \"}]\";\
         \n  output: \"#{\" foobarbaz \"}\";\
         \n  output: \'#{\"foobarbaz\"}\';\
         \n  output: \"[\'#{\" foobarbaz \"}\']\";\
         \n}\n"
    );
}
