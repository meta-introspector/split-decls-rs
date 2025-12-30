// Generated macro for exactly_n_digits (function)
macro_rules! Depcrate_parsing_combinatorexactly_n_digits {
() => {
// Module: crate::parsing::combinator
// Provides: {"exactly_n_digits"}
// Dependencies: {}
# [doc = " Consume exactly `n` digits, returning the numerical value."] # [inline] pub (crate) fn exactly_n_digits < const N : u8 , T : Integer > (input : & [u8]) -> Option < ParsedItem < '_ , T > > { n_to_m_digits :: < N , N , _ > (input) }
};
}
