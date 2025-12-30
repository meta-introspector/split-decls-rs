// Generated macro for cond (function)
macro_rules! Depcrate_combinator_corecond {
() => {
// Module: crate::combinator::core
// Provides: {"cond"}
// Dependencies: {}
# [doc = " Calls the parser if the condition is met."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::combinator::opt;"] # [doc = " use winnow::combinator::cond;"] # [doc = " use winnow::ascii::alpha1;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " fn parser<'i>(i: &mut &'i str) -> ModalResult<Option<&'i str>> {"] # [doc = "   let prefix = opt(\"-\").parse_next(i)?;"] # [doc = "   let condition = prefix.is_some();"] # [doc = "   cond(condition, alpha1).parse_next(i)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser.parse_peek(\"-abcd;\"), Ok((\";\", Some(\"abcd\"))));"] # [doc = " assert_eq!(parser.parse_peek(\"abcd;\"), Ok((\"abcd;\", None)));"] # [doc = " assert!(parser.parse_peek(\"-123;\").is_err());"] # [doc = " assert_eq!(parser.parse_peek(\"123;\"), Ok((\"123;\", None)));"] # [doc = " # }"] # [doc = " ```"] pub fn cond < Input , Output , Error , ParseNext > (cond : bool , mut parser : ParseNext ,) -> impl Parser < Input , Option < Output > , Error > where Input : Stream , ParseNext : Parser < Input , Output , Error > , Error : ParserError < Input > , { trace ("cond" , move | input : & mut Input | { if cond { parser . parse_next (input) . map (Some) } else { Ok (None) } }) }
};
}
