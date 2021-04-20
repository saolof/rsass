//! Tests auto-converted from "sass-spec/spec/core_functions/list/index.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: index(c d e)}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $value.\
         \n  ,--> input.scss\
         \n1 | a {b: index(c d e)}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function index($list, $value) {\
         \n  |           ==================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: index(c d e, d, e)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: index(c d e, d, e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function index($list, $value) {\
         \n  |           ==================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod found {
    #[test]
    fn first() {
        assert_eq!(
            crate::rsass(
                "a {b: index(a b c, a)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            crate::rsass(
                "a {b: index(a b c, c)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 3;\
        \n}\
        \n"
        );
    }
    #[test]
    fn map() {
        assert_eq!(
            crate::rsass(
                "a {b: index((c: d, e: f, g: h), e f)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2;\
        \n}\
        \n"
        );
    }
    #[test]
    fn multiple() {
        assert_eq!(
            crate::rsass(
                "a {b: index(a b c a b c, b)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2;\
        \n}\
        \n"
        );
    }
    #[test]
    fn non_list() {
        assert_eq!(
            crate::rsass(
                "a {b: index(c, c)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn sass_equality() {
        assert_eq!(
            crate::rsass(
                "a {b: index(1px 1in 1cm, 96px)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2;\
        \n}\
        \n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            crate::rsass(
                "a {b: index([c], c)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: index($list: c d e, $value: d)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2;\
        \n}\
        \n"
    );
}
mod not_found {
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(index((), c))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: null;\
        \n}\
        \n"
        );
    }
    mod map {
        #[test]
        fn empty() {
            assert_eq!(
                crate::rsass(
                    "@import \"core_functions/list/utils\";\
            \na {b: inspect(index($empty-map, e))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: null;\
        \n}\
        \n"
            );
        }
        #[test]
        fn non_empty() {
            assert_eq!(
                crate::rsass(
                    "a {b: inspect(index((c: d, e: f, g: h), e))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: null;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(index(c d e, f))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: null;\
        \n}\
        \n"
        );
    }
    #[test]
    fn non_list() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(index(c, d))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: null;\
        \n}\
        \n"
        );
    }
}