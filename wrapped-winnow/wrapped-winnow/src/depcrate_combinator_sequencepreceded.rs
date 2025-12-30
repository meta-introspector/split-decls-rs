// Generated macro for preceded (function)
macro_rules! Depcrate_combinator_sequencepreceded {
() => {
// Module: crate::combinator::sequence
// Provides: {"preceded"}
// Dependencies: {}
# [doc = " Sequence two parsers, only returning the output from the second."] # [doc = ""] # [doc = " See also [`seq`] to generalize this across any number of fields."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::{error::ErrMode, error::Needed};"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::error::Needed::Size;"] # [doc = " use winnow::combinator::preceded;"] # [doc = ""] # [doc = " fn parser<'i>(input: &mut &'i str) -> ModalResult<&'i str> {"] # [doc = "     preceded(\"abc\", \"efg\").parse_next(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser.parse_peek(\"abcefg\"), Ok((\"\", \"efg\")));"] # [doc = " assert_eq!(parser.parse_peek(\"abcefghij\"), Ok((\"hij\", \"efg\")));"] # [doc = " assert!(parser.parse_peek(\"\").is_err());"] # [doc = " assert!(parser.parse_peek(\"123\").is_err());"] # [doc = " ```"] # [doc (alias = "ignore_then")] pub fn preceded < Input , Ignored , Output , Error , IgnoredParser , ParseNext > (mut ignored : IgnoredParser , mut parser : ParseNext ,) -> impl Parser < Input , Output , Error > where Input : Stream , Error : ParserError < Input > , IgnoredParser : Parser < Input , Ignored , Error > , ParseNext : Parser < Input , Output , Error > , { trace ("preceded" , move | input : & mut Input | { let _ = ignored . parse_next (input) ? ; parser . parse_next (input) }) }
};
}
