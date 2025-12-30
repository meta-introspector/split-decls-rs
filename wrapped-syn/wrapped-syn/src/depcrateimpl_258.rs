// Generated macro for impl_258 (impl)
macro_rules! Depcrateimpl_258 {
() => {
// Module: crate
// Provides: {"impl_258"}
// Dependencies: {}
# [cfg (feature = "printing")] impl < 'a , T > quote :: ToTokens for TokensOrDefault < 'a , T > where T : quote :: ToTokens + Default , { fn to_tokens (& self , tokens : & mut quote :: Tokens) { match * self . 0 { Some (ref t) => t . to_tokens (tokens) , None => T :: default () . to_tokens (tokens) , } } }
};
}
