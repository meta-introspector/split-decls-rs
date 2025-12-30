// Generated macro for printing (module)
macro_rules! Depcrate_dataprinting {
() => {
// Module: crate::data
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use crate :: data :: { Field , FieldsNamed , FieldsUnnamed , Variant } ; use crate :: print :: TokensOrDefault ; use proc_macro2 :: TokenStream ; use quote :: { ToTokens , TokenStreamExt } ; # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for Variant { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append_all (& self . attrs) ; self . ident . to_tokens (tokens) ; self . fields . to_tokens (tokens) ; if let Some ((eq_token , disc)) = & self . discriminant { eq_token . to_tokens (tokens) ; disc . to_tokens (tokens) ; } } } # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for FieldsNamed { fn to_tokens (& self , tokens : & mut TokenStream) { self . brace_token . surround (tokens , | tokens | { self . named . to_tokens (tokens) ; }) ; } } # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for FieldsUnnamed { fn to_tokens (& self , tokens : & mut TokenStream) { self . paren_token . surround (tokens , | tokens | { self . unnamed . to_tokens (tokens) ; }) ; } } # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for Field { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append_all (& self . attrs) ; self . vis . to_tokens (tokens) ; if let Some (ident) = & self . ident { ident . to_tokens (tokens) ; TokensOrDefault (& self . colon_token) . to_tokens (tokens) ; } self . ty . to_tokens (tokens) ; } } }
};
}
