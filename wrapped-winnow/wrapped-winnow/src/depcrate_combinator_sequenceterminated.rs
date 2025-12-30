// Generated macro for terminated (function)
macro_rules! Depcrate_combinator_sequenceterminated {
() => {
// Module: crate::combinator::sequence
// Provides: {"terminated"}
// Dependencies: {}
# [doc = " Sequence two parsers, only returning the output of the first."] # [doc = ""] # [doc = " See also [`seq`] to generalize this across any number of fields."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::{error::ErrMode, error::Needed};"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::error::Needed::Size;"] # [doc = " use winnow::combinator::terminated;"] # [doc = ""] # [doc = " fn parser<'i>(input: &mut &'i str) -> ModalResult<&'i str> {"] # [doc = "     terminated(\"abc\", \"efg\").parse_next(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser.parse_peek(\"abcefg\"), Ok((\"\", \"abc\")));"] # [doc = " assert_eq!(parser.parse_peek(\"abcefghij\"), Ok((\"hij\", \"abc\")));"] # [doc = " assert!(parser.parse_peek(\"\").is_err());"] # [doc = " assert!(parser.parse_peek(\"123\").is_err());"] # [doc = " ```"] # [doc (alias = "then_ignore")] pub fn terminated < Input , Output , Ignored , Error , ParseNext , IgnoredParser > (mut parser : ParseNext , mut ignored : IgnoredParser ,) -> impl Parser < Input , Output , Error > where Input : Stream , Error : ParserError < Input > , ParseNext : Parser < Input , Output , Error > , IgnoredParser : Parser < Input , Ignored , Error > , { trace ("terminated" , move | input : & mut Input | { let o = parser . parse_next (input) ? ; ignored . parse_next (input) . map (| _ | o) }) }
};
}
