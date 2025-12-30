// Generated macro for opt (function)
macro_rules! Depcrate_parsing_combinatoropt {
() => {
// Module: crate::parsing::combinator
// Provides: {"opt"}
// Dependencies: {}
# [doc = " Optionally consume an input with a given parser."] # [inline] pub (crate) fn opt < 'a , T > (parser : impl Fn (& 'a [u8]) -> Option < ParsedItem < 'a , T > > ,) -> impl Fn (& 'a [u8]) -> ParsedItem < 'a , Option < T > > { move | input | match parser (input) { Some (value) => value . map (Some) , None => ParsedItem (input , None) , } }
};
}
