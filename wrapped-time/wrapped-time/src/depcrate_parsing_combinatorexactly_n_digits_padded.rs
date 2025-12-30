// Generated macro for exactly_n_digits_padded (function)
macro_rules! Depcrate_parsing_combinatorexactly_n_digits_padded {
() => {
// Module: crate::parsing::combinator
// Provides: {"exactly_n_digits_padded"}
// Dependencies: {}
# [doc = " Consume exactly `n` digits, returning the numerical value."] # [inline] pub (crate) fn exactly_n_digits_padded < 'a , const N : u8 , T : Integer > (padding : Padding ,) -> impl Fn (& 'a [u8]) -> Option < ParsedItem < 'a , T > > { n_to_m_digits_padded :: < N , N , _ > (padding) }
};
}
