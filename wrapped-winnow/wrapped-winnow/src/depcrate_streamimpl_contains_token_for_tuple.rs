// Generated macro for impl_contains_token_for_tuple (macro)
macro_rules! Depcrate_streamimpl_contains_token_for_tuple {
() => {
// Module: crate::stream
// Provides: {"impl_contains_token_for_tuple"}
// Dependencies: {}
macro_rules ! impl_contains_token_for_tuple { ($ ($ haystack : ident) ,+) => (# [allow (non_snake_case)] impl < T , $ ($ haystack) ,+> ContainsToken < T > for ($ ($ haystack) ,+,) where T : Clone , $ ($ haystack : ContainsToken < T >) ,+ { # [inline] fn contains_token (& self , token : T) -> bool { let ($ (ref $ haystack) ,+,) = * self ; $ ($ haystack . contains_token (token . clone ()) ||) + false } }) }
};
}
