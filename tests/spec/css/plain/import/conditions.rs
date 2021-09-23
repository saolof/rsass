//! Tests auto-converted from "sass-spec/spec/css/plain/import/conditions.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn both() {
    assert_eq!(
        runner().ok(
            "@import url(\"a.css\") supports(display: flex) handheld and (max-width: 400px);\n"
        ),
        "@import url(\"a.css\") supports(display: flex) handheld and (max-width: 400px);\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn wrong_order() {
        assert_eq!(
        runner().err(
            "@import url(\"a.css\") (max-width: 400px) supports(display: flex);\n"
        ),
        "Error: expected \";\".\
         \n  ,\
         \n1 | @import url(\"a.css\") (max-width: 400px) supports(display: flex);\
         \n  |                                         ^\
         \n  \'\
         \n  input.scss 1:41  root stylesheet",
    );
    }
}
mod media {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn complex() {
        assert_eq!(
            runner().ok(
                "@import url(\"a.css\") handheld and (max-width: 400px);\n"
            ),
            "@import url(\"a.css\") handheld and (max-width: 400px);\n"
        );
    }
    #[test]
    fn simple() {
        assert_eq!(
            runner().ok("@import url(\"a.css\") print;\n"),
            "@import url(\"a.css\") print;\n"
        );
    }
}
mod supports {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn condition() {
        assert_eq!(
            runner().ok("@import \"a.css\" supports((a: b));\n"),
            "@import \"a.css\" supports(a: b);\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn condition_and() {
        assert_eq!(
            runner().ok("@import \"a.css\" supports((a: b) and (c: d));\n"),
            "@import \"a.css\" supports((a: b) and (c: d));\n"
        );
    }
    #[test]
    fn condition_negation() {
        assert_eq!(
            runner().ok("@import \"a.css\" supports(not (a: b));"),
            "@import \"a.css\" supports(not (a: b));\n"
        );
    }
    #[test]
    fn declaration() {
        assert_eq!(
            runner().ok("@import \"a.css\" supports(a: b);\n"),
            "@import \"a.css\" supports(a: b);\n"
        );
    }
}