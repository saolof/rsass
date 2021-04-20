//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/error/units.hrx"

mod none {
    #[test]
    #[ignore] // missing error
    fn blackness() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(black, $blackness: 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $blackness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: change-color(black, $blackness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn whiteness() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(white, $whiteness: 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $whiteness: Expected 1 to have unit \"%\".\
         \n  ,\
         \n1 | a {b: change-color(white, $whiteness: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod wrong {
    #[test]
    #[ignore] // missing error
    fn blackness() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(black, $blackness: 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $blackness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: change-color(black, $blackness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn whiteness() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(white, $whiteness: 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $whiteness: Expected 1px to have unit \"%\".\
         \n  ,\
         \n1 | a {b: change-color(white, $whiteness: 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}