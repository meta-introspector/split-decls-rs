// Generated macro for impl_67 (impl)
macro_rules! Depcrate_tokenimpl_67 {
() => {
// Module: crate::token
// Provides: {"impl_67"}
// Dependencies: {}
# [cfg (feature = "printing")] # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for Underscore { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Ident :: new ("_" , self . span)) ; } }
};
}
