//! Tests auto-converted from "sass-spec/spec/css/plain/import/partial_conflict.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("_plain.css", "plain {partial: true}\n")
        .mock_file("plain.css", "plain {partial: false}\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err("@import \"plain\";\n"),
        "Error: It\'s not clear which file to import. Found:\
         \n  _plain.css\
         \n  plain.css\
         \n  ,\
         \n1 | @import \"plain\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
    );
}
