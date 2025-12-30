// Generated macro for AnyDelimiter (trait)
macro_rules! Depcrate_parse_discouragedAnyDelimiter {
() => {
// Module: crate::parse::discouraged
// Provides: {"AnyDelimiter"}
// Dependencies: {}
# [doc = " Extensions to the `ParseStream` API to support manipulating invisible"] # [doc = " delimiters the same as if they were visible."] pub trait AnyDelimiter { # [doc = " Returns the delimiter, the span of the delimiter token, and the nested"] # [doc = " contents for further parsing."] fn parse_any_delimiter (& self) -> Result < (Delimiter , DelimSpan , ParseBuffer) > ; }
};
}
