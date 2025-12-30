// Generated macro for ParseFromDescription (enum)
macro_rules! Depcrate_error_parse_from_descriptionParseFromDescription {
() => {
// Module: crate::error::parse_from_description
// Provides: {"ParseFromDescription"}
// Dependencies: {}
# [doc = " An error that occurred while parsing the input into a [`Parsed`](crate::parsing::Parsed) struct."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum ParseFromDescription { # [doc = " A string literal was not what was expected."] # [non_exhaustive] InvalidLiteral , # [doc = " A dynamic component was not valid."] InvalidComponent (& 'static str) , # [doc = " The input was expected to have ended, but there are characters that remain."] # [non_exhaustive] UnexpectedTrailingCharacters , }
};
}
