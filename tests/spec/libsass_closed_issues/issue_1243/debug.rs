//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/debug.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(runner().ok("@debug(\"foo\")\n"), "");
}
