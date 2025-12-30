// Generated macro for LineEnding (enum)
macro_rules! Depcrate_line_endingLineEnding {
() => {
// Module: crate::line_ending
// Provides: {"LineEnding"}
// Dependencies: {}
# [doc = " Supported line endings. Like in the Rust standard library, two line"] # [doc = " endings are supported: `\\r\\n` and `\\n`"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum LineEnding { # [doc = " _Carriage return and line feed_ – a line ending sequence"] # [doc = " historically used in Windows. Corresponds to the sequence"] # [doc = " of ASCII control characters `0x0D 0x0A` or `\\r\\n`"] CRLF , # [doc = " _Line feed_ – a line ending historically used in Unix."] # [doc = "  Corresponds to the ASCII control character `0x0A` or `\\n`"] LF , }
};
}
