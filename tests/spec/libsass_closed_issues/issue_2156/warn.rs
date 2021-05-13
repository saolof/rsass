//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2156/warn.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@warn \"\\\"foo\\\"\" + \"\";\r\
             \n@warn \"\" + \"\\\"foo\\\"\";\r\
             \n@warn \"\" + \"\\\"foo\";\r\
             \n@warn \"\\\"foo\\\"\" + \"bar\";\r\
             \n@warn unquote(\"\\\"foo\\\" and \\\"bar\\\"\");\r\n"),
        ""
    );
}
