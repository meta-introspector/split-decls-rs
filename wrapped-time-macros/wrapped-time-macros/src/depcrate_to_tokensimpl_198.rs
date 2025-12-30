// Generated macro for impl_198 (impl)
macro_rules! Depcrate_to_tokensimpl_198 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_198"}
// Dependencies: {}
impl ToTokenTree for NonZero < u16 > { fn into_token_tree (self) -> TokenTree { quote_group ! { { unsafe { :: core :: num :: NonZero ::< u16 >:: new_unchecked (# (self . get ())) } } } } }
};
}
