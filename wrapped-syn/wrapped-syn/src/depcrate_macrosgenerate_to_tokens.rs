// Generated macro for generate_to_tokens (macro)
macro_rules! Depcrate_macrosgenerate_to_tokens {
() => {
// Module: crate::macros
// Provides: {"generate_to_tokens"}
// Dependencies: {}
macro_rules ! generate_to_tokens { (do_not_generate_to_tokens $ ($ foo : tt) *) => () ; (enum $ name : ident { $ ($ variant : ident [$ ($ rest : tt) *] ,) * }) => (# [cfg (feature = "printing")] impl :: quote :: ToTokens for $ name { fn to_tokens (& self , tokens : & mut :: quote :: Tokens) { match * self { $ ($ name ::$ variant (ref _e) => to_tokens_call ! (_e , tokens , $ ($ rest) *) ,) * } } }) ; }
};
}
