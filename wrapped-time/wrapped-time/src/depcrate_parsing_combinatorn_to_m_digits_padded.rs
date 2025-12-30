// Generated macro for n_to_m_digits_padded (function)
macro_rules! Depcrate_parsing_combinatorn_to_m_digits_padded {
() => {
// Module: crate::parsing::combinator
// Provides: {"n_to_m_digits_padded"}
// Dependencies: {}
# [doc = " Consume between `n` and `m` digits, returning the numerical value."] # [inline] pub (crate) fn n_to_m_digits_padded < 'a , const N : u8 , const M : u8 , T : Integer > (padding : Padding ,) -> impl Fn (& 'a [u8]) -> Option < ParsedItem < 'a , T > > { debug_assert ! (M >= N) ; move | mut input | match padding { Padding :: None => n_to_m_digits :: < 1 , M , _ > (input) , Padding :: Space => { debug_assert ! (N > 0) ; let mut orig_input = input ; for _ in 0 .. (N - 1) { match ascii_char :: < b' ' > (input) { Some (parsed) => input = parsed . 0 , None => break , } } let pad_width = (orig_input . len () - input . len ()) . truncate :: < u8 > () ; orig_input = input ; for _ in 0 .. (N - pad_width) { input = any_digit (input) ? . 0 ; } for _ in N .. M { match any_digit (input) { Some (parsed) => input = parsed . 0 , None => break , } } ParsedItem (input , & orig_input [.. (orig_input . len () - input . len ())]) . flat_map (| value | value . parse_bytes ()) } Padding :: Zero => n_to_m_digits :: < N , M , _ > (input) , } }
};
}
