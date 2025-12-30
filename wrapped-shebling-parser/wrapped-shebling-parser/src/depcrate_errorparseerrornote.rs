// Generated macro for ParseErrorNote (struct)
macro_rules! Depcrate_errorParseErrorNote {
() => {
// Module: crate::error
// Provides: {"ParseErrorNote"}
// Dependencies: {}
# [doc = " Extra context information about a `shebling` parse error."] # [derive (Debug , miette :: Diagnostic , thiserror :: Error)] # [error ("{note}")] # [diagnostic (severity ("error"))] pub (crate) struct ParseErrorNote { # [label] location : usize , note : & 'static str , }
};
}
