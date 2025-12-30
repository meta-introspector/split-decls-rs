// Generated macro for ParseErrorNote (struct)
macro_rules! Depcrate_errorParseErrorNote {
() => {
// Module: crate::error
// Provides: {"ParseErrorNote"}
// Dependencies: {}
# [derive (Clone , Debug , miette :: Diagnostic , Error)] # [error ("{note}")] # [diagnostic (severity ("error"))] pub (crate) struct ParseErrorNote { # [label] location : Location , note : & 'static str , }
};
}
