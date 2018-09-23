//! Tests from `spec/parser/interpolate`

extern crate rsass;

use rsass::{compile_scss, OutputStyle};

mod t00_concatenation;
mod t01_literal;
mod t02_double_quoted;
mod t03_single_quoted;
mod t04_space_list_quoted;
mod t06_space_list_complex;
mod t10_escaped_backslash;
mod t11_escaped_literal;
mod t12_escaped_double_quoted;
mod t15_escapes_double_quoted_numbers;
mod t18_escapes_double_quoted_lowercase;
mod t24_escapes_double_quoted_specials;
mod t26_escaped_literal_quotes;
mod t27_escaped_double_quotes;
mod t31_schema_simple;

fn check(input: &str, expected: &str) {
    assert_eq!(
        compile_scss(input.as_bytes(), OutputStyle::Expanded)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        expected
    );
}