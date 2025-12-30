// Generated macro for n_to_m (function)
macro_rules! Depcrate_parsing_combinatorn_to_m {
() => {
// Module: crate::parsing::combinator
// Provides: {"n_to_m"}
// Dependencies: {}
# [doc = " Consume between `n` and `m` instances of the provided parser."] # [inline] pub (crate) fn n_to_m < 'a , const N : u8 , const M : u8 , T , P : Fn (& 'a [u8]) -> Option < ParsedItem < 'a , T > > , > (parser : P ,) -> impl Fn (& 'a [u8]) -> Option < ParsedItem < 'a , & 'a [u8] > > { debug_assert ! (M >= N) ; move | mut input | { let orig_input = input ; for _ in 0 .. N { input = parser (input) ? . 0 ; } for _ in N .. M { match parser (input) { Some (parsed) => input = parsed . 0 , None => break , } } Some (ParsedItem (input , & orig_input [.. (orig_input . len () - input . len ())] ,)) } }
};
}
