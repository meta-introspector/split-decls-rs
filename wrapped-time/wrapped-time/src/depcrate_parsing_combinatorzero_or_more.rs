// Generated macro for zero_or_more (function)
macro_rules! Depcrate_parsing_combinatorzero_or_more {
() => {
// Module: crate::parsing::combinator
// Provides: {"zero_or_more"}
// Dependencies: {}
# [doc = " Consume zero or more instances of the provided parser. The parser must return the unit value."] # [inline] pub (crate) fn zero_or_more < 'a , P : Fn (& 'a [u8]) -> Option < ParsedItem < 'a , () > > > (parser : P ,) -> impl FnMut (& 'a [u8]) -> ParsedItem < 'a , () > { move | mut input | { while let Some (remaining) = parser (input) { input = remaining . into_inner () ; } ParsedItem (input , ()) } }
};
}
