//! Tests auto-converted from "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/01_inline.hrx"

// Ignoring "t01_inline", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/02_variable.hrx"

// Ignoring "t02_variable", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/03_inline_double.hrx"

// Ignoring "t03_inline_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/04_variable_double.hrx"

// Ignoring "t04_variable_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/05_variable_quoted_double.hrx"

// Ignoring "t05_variable_quoted_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/06_escape_interpolation.hrx"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \\1\\2\\3\\4\\5\\6\\7\\8\\9;\n.result {\n  output: \"[\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}]\";\n  output: \"\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\";\n  output: \'\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\';\n  output: \"[\'\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}]\";\n  output: \"\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\";\n  output: \'\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\';\n  output: \"[\'\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\']\";\n}\n"
    );
}
