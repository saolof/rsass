//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/34_mixed_list/02_variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"[\"\',foo   ,   \'\"]\"    \"bar\";\
             \n.result {\
             \n  output: $input;\
             \n  output: #{$input};\
             \n  output: \"[#{$input}]\";\
             \n  output: \"#{$input}\";\
             \n  output: \'#{$input}\';\
             \n  output: \"[\'#{$input}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[\" \",foo   ,   \" \"]\" \"bar\";\
         \n  output: [ ,foo   ,    ] bar;\
         \n  output: \"[[ ,foo   ,    ] bar]\";\
         \n  output: \"[ ,foo   ,    ] bar\";\
         \n  output: \"[ ,foo   ,    ] bar\";\
         \n  output: \"[\'[ ,foo   ,    ] bar\']\";\
         \n}\n"
    );
}
