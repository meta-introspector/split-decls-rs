// Generated macro for separated_pair (function)
macro_rules! Depcrate_combinator_sequenceseparated_pair {
() => {
// Module: crate::combinator::sequence
// Provides: {"separated_pair"}
// Dependencies: {}
# [doc = " Sequence three parsers, only returning the values of the first and third."] # [doc = ""] # [doc = " See also [`seq`] to generalize this across any number of fields."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::{error::ErrMode, error::Needed};"] # [doc = " # use winnow::error::Needed::Size;"] # [doc = " # use winnow::prelude::*;"] # [doc = " use winnow::combinator::separated_pair;"] # [doc = ""] # [doc = " fn parser<'i>(input: &mut &'i str) -> ModalResult<(&'i str, &'i str)> {"] # [doc = "     separated_pair(\"abc\", \"|\", \"efg\").parse_next(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser.parse_peek(\"abc|efg\"), Ok((\"\", (\"abc\", \"efg\"))));"] # [doc = " assert_eq!(parser.parse_peek(\"abc|efghij\"), Ok((\"hij\", (\"abc\", \"efg\"))));"] # [doc = " assert!(parser.parse_peek(\"\").is_err());"] # [doc = " assert!(parser.parse_peek(\"123\").is_err());"] # [doc = " ```"] pub fn separated_pair < Input , O1 , Sep , O2 , Error , P1 , SepParser , P2 > (mut first : P1 , mut sep : SepParser , mut second : P2 ,) -> impl Parser < Input , (O1 , O2) , Error > where Input : Stream , Error : ParserError < Input > , P1 : Parser < Input , O1 , Error > , SepParser : Parser < Input , Sep , Error > , P2 : Parser < Input , O2 , Error > , { trace ("separated_pair" , move | input : & mut Input | { let o1 = first . parse_next (input) ? ; let _ = sep . parse_next (input) ? ; second . parse_next (input) . map (| o2 | (o1 , o2)) }) }
};
}
