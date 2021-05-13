//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/027_test_protocol_relative_import.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner()
            .ok("@import \"//fonts.googleapis.com/css?family=Droid+Sans\";"),
        "@import \"//fonts.googleapis.com/css?family=Droid+Sans\";\n"
    );
}
