//! Tests auto-converted from "sass-spec/spec/directives/forward/member/import/import_to_forward/transitive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file(
            "transitive/function/_downstream.scss",
            "@import \"midstream\";\n",
        )
        .mock_file(
            "transitive/function/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "transitive/function/_upstream.scss",
            "@function c() {@return d}\n",
        )
        .mock_file(
            "transitive/mixin/_downstream.scss",
            "@import \"midstream\";\n",
        )
        .mock_file(
            "transitive/mixin/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file("transitive/mixin/_upstream.scss", "@mixin b {c: d}\n")
        .mock_file(
            "transitive/variable/_downstream.scss",
            "@import \"midstream\";\n",
        )
        .mock_file(
            "transitive/variable/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file("transitive/variable/_upstream.scss", "$c: d;\n")
}

mod transitive {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("transitive")
    }

    #[test]
    #[ignore] // wrong result
    fn function() {
        let runner = runner().with_cwd("function");
        assert_eq!(
            runner.ok("@import \"downstream\";\n\
             \na {b: c()}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn mixin() {
        let runner = runner().with_cwd("mixin");
        assert_eq!(
            runner.ok("@import \"downstream\";\n\
             \na {@include b}\n"),
            "a {\
         \n  c: d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn variable() {
        let runner = runner().with_cwd("variable");
        assert_eq!(
            runner.ok("@import \"downstream\";\n\
             \na {b: $c}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
}