// Generated macro for opt (function)
macro_rules! Depcrate_combinator_coreopt {
() => {
// Module: crate::combinator::core
// Provides: {"opt"}
// Dependencies: {}
# [doc = " Apply a [`Parser`], producing `None` on [`ErrMode::Backtrack`][crate::error::ErrMode::Backtrack]."] # [doc = ""] # [doc = " To chain an error up, see [`cut_err`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " use winnow::combinator::opt;"] # [doc = " use winnow::ascii::alpha1;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " fn parser<'i>(i: &mut &'i str) -> ModalResult<Option<&'i str>> {"] # [doc = "   opt(alpha1).parse_next(i)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser.parse_peek(\"abcd;\"), Ok((\";\", Some(\"abcd\"))));"] # [doc = " assert_eq!(parser.parse_peek(\"123;\"), Ok((\"123;\", None)));"] # [doc = " # }"] # [doc = " ```"] pub fn opt < Input : Stream , Output , Error , ParseNext > (mut parser : ParseNext ,) -> impl Parser < Input , Option < Output > , Error > where ParseNext : Parser < Input , Output , Error > , Error : ParserError < Input > , { trace ("opt" , move | input : & mut Input | { let start = input . checkpoint () ; match parser . parse_next (input) { Ok (o) => Ok (Some (o)) , Err (e) if e . is_backtrack () => { input . reset (& start) ; Ok (None) } Err (e) => Err (e) , } }) }
};
}
