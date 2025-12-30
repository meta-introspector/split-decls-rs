// Generated macro for printing (module)
macro_rules! Depcrate_attrprinting {
() => {
// Module: crate::attr
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use crate :: attr :: { AttrStyle , Attribute , Meta , MetaList , MetaNameValue } ; use crate :: path ; use crate :: path :: printing :: PathStyle ; use proc_macro2 :: TokenStream ; use quote :: ToTokens ; # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for Attribute { fn to_tokens (& self , tokens : & mut TokenStream) { self . pound_token . to_tokens (tokens) ; if let AttrStyle :: Inner (b) = & self . style { b . to_tokens (tokens) ; } self . bracket_token . surround (tokens , | tokens | { self . meta . to_tokens (tokens) ; }) ; } } # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for Meta { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Meta :: Path (path) => path :: printing :: print_path (tokens , path , PathStyle :: Mod) , Meta :: List (meta_list) => meta_list . to_tokens (tokens) , Meta :: NameValue (meta_name_value) => meta_name_value . to_tokens (tokens) , } } } # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for MetaList { fn to_tokens (& self , tokens : & mut TokenStream) { path :: printing :: print_path (tokens , & self . path , PathStyle :: Mod) ; self . delimiter . surround (tokens , self . tokens . clone ()) ; } } # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for MetaNameValue { fn to_tokens (& self , tokens : & mut TokenStream) { path :: printing :: print_path (tokens , & self . path , PathStyle :: Mod) ; self . eq_token . to_tokens (tokens) ; self . value . to_tokens (tokens) ; } } }
};
}
