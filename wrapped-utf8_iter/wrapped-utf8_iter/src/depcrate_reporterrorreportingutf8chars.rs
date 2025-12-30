// Generated macro for ErrorReportingUtf8Chars (struct)
macro_rules! Depcrate_reportErrorReportingUtf8Chars {
() => {
// Module: crate::report
// Provides: {"ErrorReportingUtf8Chars"}
// Dependencies: {}
# [doc = " Iterator by `Result<char,Utf8CharsError>` over `&[u8]` that contains"] # [doc = " potentially-invalid UTF-8. There is exactly one `Utf8CharsError` per"] # [doc = " each error as defined by the WHATWG Encoding Standard."] # [doc = ""] # [doc = " ```"] # [doc = " let s = b\"a\\xFFb\\xFF\\x80c\\xF0\\x9F\\xA4\\xA6\\xF0\\x9F\\xA4\\xF0\\x9F\\xF0d\";"] # [doc = " let plain = utf8_iter::Utf8Chars::new(s);"] # [doc = " let reporting = utf8_iter::ErrorReportingUtf8Chars::new(s);"] # [doc = " assert!(plain.eq(reporting.map(|r| r.unwrap_or('\\u{FFFD}'))));"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct ErrorReportingUtf8Chars < 'a > { remaining : & 'a [u8] , }
};
}
