// Generated macro for impl_contains_token_for_tuples (macro)
macro_rules! Depcrate_streamimpl_contains_token_for_tuples {
() => {
// Module: crate::stream
// Provides: {"impl_contains_token_for_tuples"}
// Dependencies: {}
macro_rules ! impl_contains_token_for_tuples { ($ haystack1 : ident , $ ($ haystack : ident) ,+) => { impl_contains_token_for_tuples ! (__impl $ haystack1 ; $ ($ haystack) ,+) ; } ; (__impl $ ($ haystack : ident) ,+; $ haystack1 : ident $ (,$ haystack2 : ident) *) => { impl_contains_token_for_tuple ! ($ ($ haystack) ,+) ; impl_contains_token_for_tuples ! (__impl $ ($ haystack) ,+, $ haystack1 ; $ ($ haystack2) ,*) ; } ; (__impl $ ($ haystack : ident) ,+;) => { impl_contains_token_for_tuple ! ($ ($ haystack) ,+) ; } }
};
}
