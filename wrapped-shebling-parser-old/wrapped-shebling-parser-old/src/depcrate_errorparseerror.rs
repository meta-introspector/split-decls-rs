// Generated macro for ParseError (struct)
macro_rules! Depcrate_errorParseError {
() => {
// Module: crate::error
// Provides: {"ParseError"}
// Dependencies: {}
# [derive (Debug , Error , miette :: Diagnostic)] # [error ("parser bailed!")] # [diagnostic (code (shebling :: parser :: fatal) , severity ("error"))] pub (crate) struct ParseError { # [label ("stopped here")] location : Location , # [related] notes : Vec < ParseErrorNote > , diags : Vec < ParseDiagnostic > , }
};
}
