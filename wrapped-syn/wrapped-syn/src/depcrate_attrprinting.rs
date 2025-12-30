// Generated macro for printing (module)
macro_rules! Depcrate_attrprinting {
() => {
// Module: crate::attr
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use super :: * ; use quote :: { Tokens , ToTokens } ; impl ToTokens for Attribute { fn to_tokens (& self , tokens : & mut Tokens) { if self . is_sugared_doc { if let Some (MetaItem :: NameValue (ref pair)) = self . meta_item () { if pair . ident == "doc" { let value = pair . lit . value . to_string () ; if value . starts_with ('/') { pair . lit . to_tokens (tokens) ; return } } } } self . pound_token . to_tokens (tokens) ; if let AttrStyle :: Inner (ref b) = self . style { b . to_tokens (tokens) ; } self . bracket_token . surround (tokens , | tokens | { self . path . to_tokens (tokens) ; tokens . append_all (& self . tts) ; }) ; } } impl ToTokens for MetaItemList { fn to_tokens (& self , tokens : & mut Tokens) { self . ident . to_tokens (tokens) ; self . paren_token . surround (tokens , | tokens | { self . nested . to_tokens (tokens) ; }) } } impl ToTokens for MetaNameValue { fn to_tokens (& self , tokens : & mut Tokens) { self . ident . to_tokens (tokens) ; self . eq_token . to_tokens (tokens) ; self . lit . to_tokens (tokens) ; } } }
};
}
