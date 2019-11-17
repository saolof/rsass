//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/three_args.hrx"

mod blackness {
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0, 100%, \"foo\")}\
             \n"
            )
            .unwrap_err(),
            "Error: $blackness: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0, 100%, \"foo\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
}
mod hue {
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(\"foo\", 100%, 50%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $hue: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(\"foo\", 100%, 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
}
mod whiteness {
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.hwb(0, \"foo\", 50%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $whiteness: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0, \"foo\", 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
}
