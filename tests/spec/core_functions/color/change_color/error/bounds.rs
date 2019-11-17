//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/error/bounds.hrx"

mod alpha {
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $alpha: 1.001)}\
             \n"
            )
            .unwrap_err(),
            "Error: $alpha: Expected 1.001 to be within 0 and 1.\
         \n  ,\
         \n1 | a {b: change-color(red, $alpha: 1.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $alpha: -0.001)}\
             \n"
            )
            .unwrap_err(),
            "Error: $alpha: Expected -0.001 to be within 0 and 1.\
         \n  ,\
         \n1 | a {b: change-color(red, $alpha: -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
mod blackness {
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $blackness: 100.001%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $blackness: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $blackness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $blackness: -0.001%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $blackness: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $blackness: -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
mod blue {
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(blue, $blue: 256)}\
             \n"
            )
            .unwrap_err(),
            "Error: $blue: Expected 256 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(blue, $blue: 256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(blue, $blue: -1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $blue: Expected -1 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(blue, $blue: -1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
mod green {
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(green, $green: 256)}\
             \n"
            )
            .unwrap_err(),
            "Error: $green: Expected 256 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(green, $green: 256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(green, $green: -1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $green: Expected -1 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(green, $green: -1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
mod lightness {
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $lightness: 100.001%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $lightness: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $lightness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $lightness: -0.001%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $lightness: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $lightness: -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
mod red {
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $red: 256)}\
             \n"
            )
            .unwrap_err(),
            "Error: $red: Expected 256 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(red, $red: 256)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $red: -1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $red: Expected -1 to be within 0 and 255.\
         \n  ,\
         \n1 | a {b: change-color(red, $red: -1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
mod saturation {
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $saturation: 100.001%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $saturation: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $saturation: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $saturation: -0.001%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $saturation: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $saturation: -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
mod whiteness {
    #[test]
    #[ignore] // missing error
    fn too_high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $whiteness: 100.001%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $whiteness: Expected 100.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $whiteness: 100.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $whiteness: -0.001%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $whiteness: Expected -0.001% to be within 0% and 100%.\
         \n  ,\
         \n1 | a {b: change-color(red, $whiteness: -0.001%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
