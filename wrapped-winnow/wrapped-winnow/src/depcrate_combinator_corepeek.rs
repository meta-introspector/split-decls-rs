// Generated macro for peek (function)
macro_rules! Depcrate_combinator_corepeek {
() => {
// Module: crate::combinator::core
// Provides: {"peek"}
// Dependencies: {}
# [doc = " Apply the parser without advancing the input."] # [doc = ""] # [doc = " To lookahead and only advance on success, see [`opt`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " use winnow::combinator::peek;"] # [doc = " use winnow::ascii::alpha1;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " fn parser<'i>(input: &mut &'i str) -> ModalResult<&'i str> {"] # [doc = "     peek(alpha1).parse_next(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser.parse_peek(\"abcd;\"), Ok((\"abcd;\", \"abcd\")));"] # [doc = " assert!(parser.parse_peek(\"123;\").is_err());"] # [doc = " # }"] # [doc = " ```"] # [doc (alias = "look_ahead")] # [doc (alias = "rewind")] pub fn peek < Input , Output , Error , ParseNext > (mut parser : ParseNext ,) -> impl Parser < Input , Output , Error > where Input : Stream , Error : ParserError < Input > , ParseNext : Parser < Input , Output , Error > , { trace ("peek" , move | input : & mut Input | { let start = input . checkpoint () ; let res = parser . parse_next (input) ; input . reset (& start) ; res }) }
};
}
