// Generated macro for printing (module)
macro_rules! Depcrate_restrictionprinting {
() => {
// Module: crate::restriction
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use crate :: path ; use crate :: path :: printing :: PathStyle ; use crate :: restriction :: { VisRestricted , Visibility } ; use proc_macro2 :: TokenStream ; use quote :: ToTokens ; # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for Visibility { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Visibility :: Public (pub_token) => pub_token . to_tokens (tokens) , Visibility :: Restricted (vis_restricted) => vis_restricted . to_tokens (tokens) , Visibility :: Inherited => { } } } } # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for VisRestricted { fn to_tokens (& self , tokens : & mut TokenStream) { self . pub_token . to_tokens (tokens) ; self . paren_token . surround (tokens , | tokens | { self . in_token . to_tokens (tokens) ; path :: printing :: print_path (tokens , & self . path , PathStyle :: Mod) ; }) ; } } }
};
}
