// Generated macro for ErrorReportingUtf16Chars (struct)
macro_rules! Depcrate_reportErrorReportingUtf16Chars {
() => {
// Module: crate::report
// Provides: {"ErrorReportingUtf16Chars"}
// Dependencies: {}
# [doc = " Iterator by `Result<char,Utf16CharsError>` over `&[u16]` that contains"] # [doc = " potentially-invalid UTF-16. There is exactly one `Utf16CharsError` per"] # [doc = " each unpaired surrogate."] # [derive (Debug , Clone)] pub struct ErrorReportingUtf16Chars < 'a > { remaining : & 'a [u16] , }
};
}
