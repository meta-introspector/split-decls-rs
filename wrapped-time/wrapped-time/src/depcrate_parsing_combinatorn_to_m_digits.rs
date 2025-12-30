// Generated macro for n_to_m_digits (function)
macro_rules! Depcrate_parsing_combinatorn_to_m_digits {
() => {
// Module: crate::parsing::combinator
// Provides: {"n_to_m_digits"}
// Dependencies: {}
# [doc = " Consume between `n` and `m` digits, returning the numerical value."] # [inline] pub (crate) fn n_to_m_digits < const N : u8 , const M : u8 , T : Integer > (input : & [u8] ,) -> Option < ParsedItem < '_ , T > > { debug_assert ! (M >= N) ; n_to_m :: < N , M , _ , _ > (any_digit) (input) ? . flat_map (| value | value . parse_bytes ()) }
};
}
