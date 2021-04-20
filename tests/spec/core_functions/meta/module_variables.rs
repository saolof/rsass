//! Tests auto-converted from "sass-spec/spec/core_functions/meta/module_variables.hrx"

#[test]
#[ignore] // unexepected error
fn test_as() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"other\" as a;\
            \n\
            \nb {c: meta.inspect(meta.module-variables(\"a\"))}\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: (\"d\": d value, \"e\": e value, \"f\": f value);\
        \n}\
        \n"
    );
}
#[test]
fn core_module() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"meta\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: ();\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn dash_sensitive() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"other\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (\"c-d\": c-d value, \"e-f\": e_f value);\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn empty() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"other\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: ();\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn before_load() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n\
             \n$variables: meta.module-variables(\"other\");\
             \n\
             \n@use \"other\";\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n3 | $variables: meta.module-variables(\"other\");\
         \n  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:13  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn dash_sensitive() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@use \"other-module\";\
             \n\
             \na {b: meta.inspect(meta.module-variables(\"other_module\"))}\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with namespace \"other_module\".\
         \n  ,\
         \n4 | a {b: meta.inspect(meta.module-variables(\"other_module\"))}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:20  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn global() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@use \"other\" as *;\
             \n\
             \na {b: meta.inspect(meta.module-variables(\"other\"))}\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n4 | a {b: meta.inspect(meta.module-variables(\"other\"))}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:20  root stylesheet",
        );
    }
    #[test]
    fn missing() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \na {b: meta.inspect(meta.module-variables(\"other\"))}\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n2 | a {b: meta.inspect(meta.module-variables(\"other\"))}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:20  root stylesheet",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \na {b: meta.inspect(meta.module-variables())}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $module.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.inspect(meta.module-variables())}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function module-variables($module) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:20  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \na {b: meta.inspect(meta.module-variables(\"meta\", \"c\"))}\
             \n"
        ).unwrap_err(),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.inspect(meta.module-variables(\"meta\", \"c\"))}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function module-variables($module) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:20  root stylesheet",
    );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \na {b: meta.inspect(meta.module-variables(1))}\
             \n"
            )
            .unwrap_err(),
            "Error: $module: 1 is not a string.\
         \n  ,\
         \n2 | a {b: meta.inspect(meta.module-variables(1))}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:20  root stylesheet",
        );
    }
}
#[test]
#[ignore] // unexepected error
fn multiple() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"other\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"other\";\
            \n\
            \na {b: meta.inspect(meta.module-variables($module: \"other\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
        \n}\
        \n"
    );
}
mod through_forward {
    #[test]
    #[ignore] // unexepected error
    fn test_as() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (\"c-d\": d value, \"c-e\": e value, \"c-f\": f value);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn bare() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hide() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (\"e\": e value);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn show() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (\"c\": c value, \"d\": d value);\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn through_import() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"used\";\
            \n\
            \na {b: meta.inspect(meta.module-variables(\"used\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
        \n}\
        \n"
    );
}