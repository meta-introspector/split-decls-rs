// Generated macro for to_predicate (function)
macro_rules! Depcrate_parseto_predicate {
() => {
// Module: crate::parse
// Provides: {"to_predicate"}
// Dependencies: {}
fn to_predicate (index : usize , peek : & PeekItem) -> Result < TokenStream > { let peek_ident : Ident = match index { 0 => parse_quote ! (peek) , 1 => parse_quote ! (peek2) , 2 => parse_quote ! (peek3) , _ => bail ! (peek . span , "more than three `#[parse(peek)]` was specified.") , } ; let peek_arg = & peek . arg ; Ok (quote ! (input .# peek_ident (# peek_arg))) }
};
}
