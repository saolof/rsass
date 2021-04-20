//! Tests auto-converted from "sass-spec/spec/core_functions/string/unquote.hrx"

#[test]
fn empty() {
    assert_eq!(
        crate::rsass(
            "$result: unquote(\"\");\
            \na {\
            \n  result: $result; // This will not be emitted because the contents is empty.\
            \n  length: str-length($result);\
            \n  same: $result == \"\";\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  length: 0;\
        \n  same: true;\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: unquote()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $string.\
         \n  ,--> input.scss\
         \n1 | a {b: unquote()}\
         \n  |       ^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function unquote($string) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: unquote(c, d)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: unquote(c, d)}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function unquote($string) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: unquote(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $string: 1 is not a string.\
         \n  ,\
         \n1 | a {b: unquote(1)}\
         \n  |       ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn escaped_backslash() {
    assert_eq!(
        crate::rsass(
            "$result: unquote(\"\\\\0 \");\
            \na {\
            \n  result: $result;\
            \n  length: str-length($result);\
            \n  same-as-argument: $result == \"\\\\0 \";\
            \n  same-as-literal: $result == \\0 ;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  result: \\0 ;\
        \n  length: 3;\
        \n  same-as-argument: true;\
        \n  same-as-literal: true;\
        \n}\
        \n"
    );
}
mod escaped_quotes {
    #[test]
    fn quoted() {
        assert_eq!(
        crate::rsass(
            "// Unquoting a quoted string returns an unquoted string with the same code\
            \n// points. Code points such as quotes that need to be escaped in the original\
            \n// don\'t need escaping in the output.\
            \n$result: unquote(\"\\\"c\\\"\");\
            \na {\
            \n  result: $result;\
            \n  length: str-length($result);\
            \n  same: $result == \"\\\"c\\\"\";\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  result: \"c\";\
        \n  length: 3;\
        \n  same: true;\
        \n}\
        \n"
    );
    }
    #[test]
    fn unquoted() {
        assert_eq!(
        crate::rsass(
            "// Unquoting an unquoted string returns it exactly as-is, leaving escapes\
            \n// totally unchanged (whether they\'re quotes or not).\
            \n$result: unquote(\\\"c\\\");\
            \na {\
            \n  result: $result;\
            \n  length: str-length($result);\
            \n  same: $result == \\\"c\\\";\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  result: \\\"c\\\";\
        \n  length: 5;\
        \n  same: true;\
        \n}\
        \n"
    );
    }
}
#[test]
fn meaningful_css_characters() {
    assert_eq!(
        crate::rsass(
            "// Unquoted strings aren\'t required to be valid CSS identifiers, and the\
            \n// `unquote()` function does *not* escape characters that aren\'t valid\
            \n// identifier characters. This allows it to be used as an escape hatch to\
            \n// produce CSS that Sass doesn\'t otherwise support.\
            \n$result: unquote(\"b; c {d: e\");\
            \na {\
            \n  result: $result;\
            \n  length: str-length($result);\
            \n  same: $result == \"b; c {d: e\";\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  result: b; c {d: e;\
        \n  length: 10;\
        \n  same: true;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: unquote($string: c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
#[test]
fn quoted() {
    assert_eq!(
        crate::rsass(
            "a {b: unquote(\"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
#[test]
fn unquoted() {
    assert_eq!(
        crate::rsass(
            "a {b: unquote(c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}