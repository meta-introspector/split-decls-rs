// Generated macro for ParseError (struct)
macro_rules! Depcrate_errorParseError {
() => {
// Module: crate::error
// Provides: {"ParseError"}
// Dependencies: {}
# [doc = " A `shebling` parse error."] # [derive (Debug , miette :: Diagnostic , thiserror :: Error)] # [error ("parser bailed!")] # [diagnostic (code (shebling :: parser :: fatal) , severity ("error"))] pub (crate) struct ParseError { # [label ("stopped here")] location : usize , # [related] notes : Vec < ParseErrorNote > , }
};
}
