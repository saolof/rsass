//! Tests auto-converted from "sass-spec/spec/core_functions/map/values.hrx"

#[test]
fn empty() {
    assert_eq!(
        crate::rsass(
            "$result: map-values(());\
            \na {\
            \n  value: inspect($result);\
            \n  separator: list-separator($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: ();\
        \n  separator: comma;\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: map-values()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $map.\
         \n  ,--> input.scss\
         \n1 | a {b: map-values()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:map\
         \n1 | @function values($map) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: map-values((c: d), (e: f))}\
             \n\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: map-values((c: d), (e: f))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:map\
         \n1 | @function values($map) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: map-values(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $map: 1 is not a map.\
         \n  ,\
         \n1 | a {b: map-values(1)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn multiple() {
    assert_eq!(
        crate::rsass(
            "a {b: map-values((c: d, e: f, g: h))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: d, f, h;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: map-values($map: (1: 2, 3: 4))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2, 4;\
        \n}\
        \n"
    );
}
#[test]
fn single() {
    assert_eq!(
        crate::rsass(
            "$result: map-values((1: 2));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n  separator: list-separator($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: 2;\
        \n  type: list;\
        \n  separator: comma;\
        \n}\
        \n"
    );
}