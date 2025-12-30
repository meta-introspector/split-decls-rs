// Generated macro for fill (function)
macro_rules! Depcrate_combinator_multifill {
() => {
// Module: crate::combinator::multi
// Provides: {"fill"}
// Dependencies: {}
# [doc = " Repeats the embedded parser, filling the given slice with results."] # [doc = ""] # [doc = " This parser fails if the input runs out before the given slice is full."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::{error::ErrMode, error::Needed};"] # [doc = " # use winnow::prelude::*;"] # [doc = " use winnow::combinator::fill;"] # [doc = ""] # [doc = " fn parser<'i>(s: &mut &'i str) -> ModalResult<[&'i str; 2]> {"] # [doc = "   let mut buf = [\"\", \"\"];"] # [doc = "   fill(\"abc\", &mut buf).parse_next(s)?;"] # [doc = "   Ok(buf)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser.parse_peek(\"abcabc\"), Ok((\"\", [\"abc\", \"abc\"])));"] # [doc = " assert!(parser.parse_peek(\"abc123\").is_err());"] # [doc = " assert!(parser.parse_peek(\"123123\").is_err());"] # [doc = " assert!(parser.parse_peek(\"\").is_err());"] # [doc = " assert_eq!(parser.parse_peek(\"abcabcabc\"), Ok((\"abc\", [\"abc\", \"abc\"])));"] # [doc = " ```"] pub fn fill < 'i , Input , Output , Error , ParseNext > (mut parser : ParseNext , buf : & 'i mut [Output] ,) -> impl Parser < Input , () , Error > + 'i where Input : Stream + 'i , ParseNext : Parser < Input , Output , Error > + 'i , Error : ParserError < Input > + 'i , { trace ("fill" , move | i : & mut Input | { for elem in buf . iter_mut () { let start = i . checkpoint () ; match parser . parse_next (i) { Ok (o) => { * elem = o ; } Err (e) => { return Err (e . append (i , & start)) ; } } } Ok (()) }) }
};
}
