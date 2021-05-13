//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/065_test_mixin_empty_var_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($a, $b...) {\
             \n  a: $a;\
             \n  b: length($b);\
             \n}\n\
             \n.foo {@include foo(1)}\n"),
        ".foo {\
         \n  a: 1;\
         \n  b: 0;\
         \n}\n"
    );
}
