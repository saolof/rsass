//! Tests auto-converted from "sass-spec/spec/core_functions/color/whiteness.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.whiteness()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.whiteness()}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function whiteness($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.whiteness(red, green)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.whiteness(red, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function whiteness($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.whiteness(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.whiteness(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
}
#[test]
#[ignore] // wrong result
fn fraction() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.whiteness(color.hwb(0, 0.5%, 0%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.3921568627%;\
        \n}\
        \n"
    );
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.whiteness(white)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 100%;\
        \n}\
        \n"
    );
}
mod middle {
    #[test]
    #[ignore] // wrong result
    fn half_blackness() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
            \na {b: color.whiteness(color.hwb(0, 50%, 50%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 50.1960784314%;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn high_blackness() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
            \na {b: color.whiteness(color.hwb(0, 70%, 70%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 50.1960784314%;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn zero_blackness() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
            \na {b: color.whiteness(color.hwb(0, 50%, 0%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 50.1960784314%;\
        \n}\
        \n"
        );
    }
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.whiteness(black)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0%;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.whiteness($color: color.hwb(0, 42%, 0%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 41.9607843137%;\
        \n}\
        \n"
    );
}
