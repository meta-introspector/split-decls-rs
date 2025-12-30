// Generated macro for one_or_more (function)
macro_rules! Depcrate_parsing_combinatorone_or_more {
() => {
// Module: crate::parsing::combinator
// Provides: {"one_or_more"}
// Dependencies: {}
# [doc = " Consume one of or more instances of the provided parser. The parser must produce the unit value."] # [inline] pub (crate) fn one_or_more < 'a , P : Fn (& 'a [u8]) -> Option < ParsedItem < 'a , () > > > (parser : P ,) -> impl Fn (& 'a [u8]) -> Option < ParsedItem < 'a , () > > { move | mut input | { input = parser (input) ? . into_inner () ; while let Some (remaining) = parser (input) { input = remaining . into_inner () ; } Some (ParsedItem (input , ())) } }
};
}
