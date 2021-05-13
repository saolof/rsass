//! Tests auto-converted from "sass-spec/spec/core_functions/selector/parse/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn inner_comma() {
    assert_eq!(
        runner().err(
            "a {b: selector-parse(((c,),))}\n"
        ),
        "Error: $selector: ((c,),) is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-parse(((c,),))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn outer_space() {
    assert_eq!(
        runner().err("a {b: selector-parse(append((), append((), c)))}\n"),
        "Error: $selector: c is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-parse(append((), append((), c)))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn parent() {
    assert_eq!(
        runner().err("a {b: selector-parse(\"&\")}\n"),
        "Error: $selector: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-parse(\"&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
mod parse {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // wrong error
    fn extra() {
        assert_eq!(
            runner().err("a {b: selector-parse(\"c {\")}\n"),
            "Error: $selector: expected selector.\
         \n  ,\
         \n1 | c {\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-parse(\"c {\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn invalid() {
        assert_eq!(
            runner().err("a {b: selector-parse(\"[c\")}\n"),
            "Error: $selector: expected more input.\
         \n  ,\
         \n1 | [c\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-parse(\"[c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn too_few_args() {
    assert_eq!(
        runner().err("a {b: selector-parse()}\n"),
        "Error: Missing argument $selector.\
         \n  ,--> input.scss\
         \n1 | a {b: selector-parse()}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function parse($selector) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn too_many_args() {
    assert_eq!(
        runner().err("a {b: selector-parse(c, d)}\n"),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: selector-parse(c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function parse($selector) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn too_nested() {
    assert_eq!(
        runner().err(
            "a {b: selector-parse((append((), append((), c)),))}\n"
        ),
        "Error: $selector: (c,) is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-parse((append((), append((), c)),))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn test_type() {
    assert_eq!(
        runner().err("a {b: selector-parse(1)}\n"),
        "Error: $selector: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-parse(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
