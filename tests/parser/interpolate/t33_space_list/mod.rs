//! Tests auto-converted from "sass-spec/spec/parser/interpolate/33_space_list"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/parser/interpolate/33_space_list/01_inline"
#[test]
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\n  output: \"[\"\'foo   \'\"]\"    \"bar\";\n  output: #{\"[\"\'foo   \'\"]\"    \"bar\"};\n  output: \"[#{\"[\"\'foo   \'\"]\"    \"bar\"}]\";\n  output: \"#{\"[\"\'foo   \'\"]\"    \"bar\"}\";\n  output: \'#{\"[\"\'foo   \'\"]\"    \"bar\"}\';\n  output: \"[\'#{\"[\"\'foo   \'\"]\"    \"bar\"}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\" \"foo   \" \"]\" \"bar\";\n  output: [ foo    ] bar;\n  output: \"[[ foo    ] bar]\";\n  output: \"[ foo    ] bar\";\n  output: \"[ foo    ] bar\";\n  output: \"[\'[ foo    ] bar\']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/33_space_list/02_variable"
#[test]
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: \"[\"\'foo   \'\"]\"    \"bar\";\n.result {\n  output: $input;\n  output: #{$input};\n  output: \"[#{$input}]\";\n  output: \"#{$input}\";\n  output: \'#{$input}\';\n  output: \"[\'#{$input}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\" \"foo   \" \"]\" \"bar\";\n  output: [ foo    ] bar;\n  output: \"[[ foo    ] bar]\";\n  output: \"[ foo    ] bar\";\n  output: \"[ foo    ] bar\";\n  output: \"[\'[ foo    ] bar\']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/33_space_list/03_inline_double"
#[test]
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\n  output: #{#{\"[\"\'foo   \'\"]\"    \"bar\"}};\n  output: #{\"[#{\"[\"\'foo   \'\"]\"    \"bar\"}]\"};\n  output: #{\"#{\"[\"\'foo   \'\"]\"    \"bar\"}\"};\n  output: #{\'#{\"[\"\'foo   \'\"]\"    \"bar\"}\'};\n  output: #{\"[\'#{\"[\"\'foo   \'\"]\"    \"bar\"}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: [ foo    ] bar;\n  output: [[ foo    ] bar];\n  output: [ foo    ] bar;\n  output: [ foo    ] bar;\n  output: [\'[ foo    ] bar\'];\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/33_space_list/04_variable_double"
#[test]
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \"[\"\'foo   \'\"]\"    \"bar\";\n.result {\n  output: #{#{$input}};\n  output: #{\"[#{$input}]\"};\n  output: #{\"#{$input}\"};\n  output: #{\'#{$input}\'};\n  output: #{\"[\'#{$input}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: [ foo    ] bar;\n  output: [[ foo    ] bar];\n  output: [ foo    ] bar;\n  output: [ foo    ] bar;\n  output: [\'[ foo    ] bar\'];\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/33_space_list/05_variable_quoted_double"
#[test]
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \"[\"\'foo   \'\"]\"    \"bar\";\n.result {\n  dquoted: \"#{#{$input}}\";\n  dquoted: \"#{\"[#{$input}]\"}\";\n  dquoted: \"#{\"#{$input}\"}\";\n  dquoted: \"#{\'#{$input}\'}\";\n  dquoted: \"#{\"[\'#{$input}\']\"}\";\n  squoted: \'#{#{$input}}\';\n  squoted: \'#{\"[#{$input}]\"}\';\n  squoted: \'#{\"#{$input}\"}\';\n  squoted: \'#{\'#{$input}\'}\';\n  squoted: \'#{\"[\'#{$input}\']\"}\';\n}\n"
        )
        .unwrap(),
        ".result {\n  dquoted: \"[ foo    ] bar\";\n  dquoted: \"[[ foo    ] bar]\";\n  dquoted: \"[ foo    ] bar\";\n  dquoted: \"[ foo    ] bar\";\n  dquoted: \"[\'[ foo    ] bar\']\";\n  squoted: \"[ foo    ] bar\";\n  squoted: \"[[ foo    ] bar]\";\n  squoted: \"[ foo    ] bar\";\n  squoted: \"[ foo    ] bar\";\n  squoted: \"[\'[ foo    ] bar\']\";\n}\n"
    );
}
