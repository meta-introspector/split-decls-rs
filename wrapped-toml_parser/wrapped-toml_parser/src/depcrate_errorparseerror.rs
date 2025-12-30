// Generated macro for ParseError (struct)
macro_rules! Depcrate_errorParseError {
() => {
// Module: crate::error
// Provides: {"ParseError"}
// Dependencies: {}
# [derive (Clone , PartialEq , Eq , Debug)] # [non_exhaustive] pub struct ParseError { context : Option < Span > , description : ErrorStr , expected : Option < & 'static [Expected] > , unexpected : Option < Span > , }
};
}
