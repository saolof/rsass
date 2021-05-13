//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1266/min.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "$foo: 1 2 3 blah 4;\
             \nfoo {\
             \n  bar: call(min, $foo...);\
             \n}\n"
        ),
        "DEPRECATION WARNING: Passing a string to call() is deprecated and will be illegal\
         \nin Dart Sass 2.0.0. Use call(get-function(min)) instead.\n\
         \n  ,\
         \n3 |   bar: call(min, $foo...);\
         \n  |        ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 3:8  root stylesheet\n\
         \nError: blah is not a number.\
         \n  ,\
         \n3 |   bar: call(min, $foo...);\
         \n  |        ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:8  root stylesheet",
    );
}
