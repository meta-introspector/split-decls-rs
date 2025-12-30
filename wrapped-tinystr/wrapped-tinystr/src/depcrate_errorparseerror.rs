// Generated macro for ParseError (enum)
macro_rules! Depcrate_errorParseError {
() => {
// Module: crate::error
// Provides: {"ParseError"}
// Dependencies: {}
# [derive (Display , Debug , PartialEq , Eq)] # [non_exhaustive] pub enum ParseError { # [displaydoc ("found string of larger length {len} when constructing string of length {max}")] TooLong { max : usize , len : usize } , # [displaydoc ("tinystr types do not support strings with null bytes")] ContainsNull , # [displaydoc ("attempted to construct TinyAsciiStr from a non-ASCII string")] NonAscii , }
};
}
