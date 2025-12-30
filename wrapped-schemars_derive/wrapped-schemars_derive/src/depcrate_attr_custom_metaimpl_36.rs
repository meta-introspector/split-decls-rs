// Generated macro for impl_36 (impl)
macro_rules! Depcrate_attr_custom_metaimpl_36 {
() => {
// Module: crate::attr::custom_meta
// Provides: {"impl_36"}
// Dependencies: {}
impl ToTokens for CustomMeta { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { match self { CustomMeta :: Not (not , path) => { not . to_tokens (tokens) ; path . to_tokens (tokens) ; } CustomMeta :: Path (meta) => meta . to_tokens (tokens) , CustomMeta :: List (meta) => meta . to_tokens (tokens) , CustomMeta :: NameValue (meta) => meta . to_tokens (tokens) , } } }
};
}
