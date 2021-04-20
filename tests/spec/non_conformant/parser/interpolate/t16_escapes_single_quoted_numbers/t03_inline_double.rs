//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/16_escapes_single_quoted_numbers/03_inline_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: #{#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}};\
            \n  output: #{\"[#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}]\"};\
            \n  output: #{\"#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}\"};\
            \n  output: #{\'#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}\'};\
            \n  output: #{\"[\'#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
        \n  output: [\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t];\
        \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
        \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
        \n  output: [\'\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t\'];\
        \n}\
        \n"
    );
}