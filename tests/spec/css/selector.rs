//! Tests auto-converted from "sass-spec/spec/css/selector.hrx"

mod attribute {
    #[test]
    fn dash_dash() {
        assert_eq!(
        crate::rsass(
            "// Attribute selector values are allowed to be unquoted as long as they\'re plain\
            \n// CSS identifiers. However, IE 11 doesn\'t recognize custom-property-style\
            \n// identifiers like `--foo` as identifiers, so they should always be quoted.\
            \n\
            \n[class=\"--foo\"], [class*=\"--foo\"] {\
            \n  x: y;\
            \n}\
            \n"
        )
        .unwrap(),
        "[class=\"--foo\"], [class*=\"--foo\"] {\
        \n  x: y;\
        \n}\
        \n"
    );
    }
    mod modifier {
        #[test]
        fn after_string() {
            assert_eq!(
                crate::rsass(
                    "[a=\"b\"i] {c: d}\
            \n"
                )
                .unwrap(),
                "[a=b i] {\
        \n  c: d;\
        \n}\
        \n"
            );
        }
        #[test]
        fn caps() {
            assert_eq!(
                crate::rsass(
                    "[a=b I] {c: d}\
            \n"
                )
                .unwrap(),
                "[a=b I] {\
        \n  c: d;\
        \n}\
        \n"
            );
        }
        #[test]
        fn unknown() {
            assert_eq!(
        crate::rsass(
            "// At time of writing, only the modifiers \"i\" and \"s\" are allowed by the CSS\
            \n// spec. However, for forwards-compatibility with future CSS additions, any\
            \n// single character should be allowed.\
            \n[a=b c] {d: e}\
            \n"
        )
        .unwrap(),
        "[a=b c] {\
        \n  d: e;\
        \n}\
        \n"
    );
        }
    }
    #[test]
    fn quoted_non_identifier() {
        assert_eq!(
        crate::rsass(
            "// Quotes should be preserved when the string they contain is not an identifier.\
            \n// See https://github.com/sass/dart-sass/issues/598.\
            \n[a=\"b.\"] {c: d}\
            \n"
        )
        .unwrap(),
        "[a=\"b.\"] {\
        \n  c: d;\
        \n}\
        \n"
    );
    }
}
mod error {
    mod attribute {
        mod modifier {
            #[test]
            #[ignore] // wrong error
            fn digit() {
                assert_eq!(
        crate::rsass(
            "// Attribute modifiers must be ASCII alphabetical characters.\
             \n[a=b 1] {c: d}\
             \n"
        ).unwrap_err(),
        "Error: expected \"]\".\
         \n  ,\
         \n2 | [a=b 1]{c: d}\
         \n  |      ^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
            }
            #[test]
            #[ignore] // wrong error
            fn no_operator() {
                assert_eq!(
                    crate::rsass(
                        "[a b] {c: d}\
             \n"
                    )
                    .unwrap_err(),
                    "Error: Expected \"]\".\
         \n  ,\
         \n1 | [a b]{c: d}\
         \n  |    ^\
         \n  \'\
         \n  input.scss 1:4  root stylesheet",
                );
            }
            #[test]
            #[ignore] // wrong error
            fn too_long() {
                assert_eq!(
                    crate::rsass(
                        "// Attribute modifiers must be single characters.\
             \n[a=b cd] {e: f}\
             \n"
                    )
                    .unwrap_err(),
                    "Error: expected \"]\".\
         \n  ,\
         \n2 | [a=b cd]{e: f}\
         \n  |       ^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
                );
            }
            #[test]
            #[ignore] // wrong error
            fn underscore() {
                assert_eq!(
        crate::rsass(
            "// Attribute modifiers must be ASCII alphabetical characters.\
             \n[a=b _] {c: d}\
             \n"
        ).unwrap_err(),
        "Error: expected \"]\".\
         \n  ,\
         \n2 | [a=b _]{c: d}\
         \n  |      ^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
            }
            #[test]
            #[ignore] // wrong error
            fn unicode() {
                assert_eq!(
        crate::rsass(
            "// Attribute modifiers must be ASCII alphabetical characters.\
             \n[a=b ï] {c: d}\
             \n"
        ).unwrap_err(),
        "Error: expected \"]\".\
         \n  ,\
         \n2 | [a=b ï]{c: d}\
         \n  |      ^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
            }
        }
    }
}
mod escaping {
    #[test]
    fn dollar_char() {
        assert_eq!(
            crate::rsass(
                ".u\\$ {a: b;}\
            \n"
            )
            .unwrap(),
            ".u\\$ {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
    #[test]
    fn dollar_char_as_numeric() {
        assert_eq!(
            crate::rsass(
                ".u\\24 {a: b;}\
            \n"
            )
            .unwrap(),
            ".u\\$ {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn number_as_first_char_with_space() {
        assert_eq!(
            crate::rsass(
                ".\\31 u {a: b;}\
            \n"
            )
            .unwrap(),
            ".\\31 u {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn number_as_first_char_without_space() {
        assert_eq!(
            crate::rsass(
                ".\\31u {a: b;}\
            \n"
            )
            .unwrap(),
            ".\\31 u {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
    #[test]
    fn number_as_nonfirst_char_with_space() {
        assert_eq!(
            crate::rsass(
                ".a\\31 u {a: b;}\
            \n"
            )
            .unwrap(),
            ".a1u {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
    #[test]
    fn number_as_nonfirst_char_without_space() {
        assert_eq!(
            crate::rsass(
                ".a\\31u {a: b;}\
            \n"
            )
            .unwrap(),
            ".a1u {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
    #[test]
    fn parenthesis_in_interpolation() {
        assert_eq!(
            crate::rsass(
                ".u#{\'\\\\28\'} { a: b; }\
            \n"
            )
            .unwrap(),
            ".u\\( {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
}
mod inline_comments {
    mod loud {}
    mod silent {}
}
mod placeholder {
    mod pseudoselectors {
        mod matches {
            #[test]
            #[ignore] // wrong result
            fn solo() {
                assert_eq!(
        crate::rsass(
            "// Since `%b` doesn\'t exist, no selectors can match it, so this rule should be\
            \n// removed.\
            \na:matches(%b) {x: y}\
            \n"
        )
        .unwrap(),
        ""
    );
            }
            #[test]
            #[ignore] // wrong result
            fn with_real() {
                assert_eq!(
        crate::rsass(
            "// Since `%b` doesn\'t exist, an element matches `%b` or `c` iff it matches `c`.\
            \na:matches(%b, c) {x: y}\
            \n"
        )
        .unwrap(),
        "a:matches(c) {\
        \n  x: y;\
        \n}\
        \n"
    );
            }
        }
        mod not {
            #[test]
            #[ignore] // wrong result
            fn solo() {
                assert_eq!(
        crate::rsass(
            "// Since `%b` doesn\'t exist, all `a` elements match `a:not(%b)`.\
            \na:not(%b) {x: y}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  x: y;\
        \n}\
        \n"
    );
            }
            #[test]
            #[ignore] // wrong result
            fn universal() {
                assert_eq!(
        crate::rsass(
            "// Since `%b` doesn\'t exist, all elements match `:not(%b)`.\
            \n:not(%b) {x: y}\
            \n"
        )
        .unwrap(),
        "* {\
        \n  x: y;\
        \n}\
        \n"
    );
            }
            #[test]
            #[ignore] // wrong result
            fn with_real() {
                assert_eq!(
        crate::rsass(
            "// Since `%b` doesn\'t exist, it can be removed from the `:not` pseudoselector.\
            \na:not(%b, c) {x: y}\
            \n"
        )
        .unwrap(),
        "a:not(c) {\
        \n  x: y;\
        \n}\
        \n"
    );
            }
        }
    }
}
mod pseudoselector {
    mod nested {
        #[test]
        fn adjacent_combinators() {
            assert_eq!(
                crate::rsass(
                    "// Regression test for sass/dart-sass#1038\
            \na {\
            \n  b:c, > d {x: y}\
            \n}\
            \n"
                )
                .unwrap(),
                "a b:c, a > d {\
        \n  x: y;\
        \n}\
        \n"
            );
        }
    }
}
#[test]
#[ignore] // wrong error
fn reference_combinator() {
    assert_eq!(
        crate::rsass(
            "// Reference combinators used to be supported by Sass when they were part of the\
             \n// CSS spec, but they\'re no longer supported and should now produce errors.\
             \n.foo /bar/ .baz {\
             \n  a: b;\
             \n}\
             \n"
        ).unwrap_err(),
        "Error: expected selector.\
         \n  ,\
         \n3 | .foo /bar/ .baz{\
         \n  |      ^\
         \n  \'\
         \n  input.scss 3:6  root stylesheet",
    );
}
#[test]
#[ignore] // unexepected error
fn slotted() {
    assert_eq!(
        crate::rsass(
            "::slotted(.a) {x: y}\
            \n\
            \n::slotted(.c.d) {x: y}\
            \n.e {@extend .c}\
            \n\
            \n::slotted(.f) {x: y}\
            \n::slotted(.g) {@extend .f}\
            \n"
        )
        .unwrap(),
        "::slotted(.a) {\
        \n  x: y;\
        \n}\
        \n::slotted(.c.d, .d.e) {\
        \n  x: y;\
        \n}\
        \n::slotted(.f, ::slotted(.g)) {\
        \n  x: y;\
        \n}\
        \n"
    );
}