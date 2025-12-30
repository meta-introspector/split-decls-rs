// Generated macro for printing (module)
macro_rules! Depcrate_deriveprinting {
() => {
// Module: crate::derive
// Provides: {"printing"}
// Dependencies: {}
# [cfg (feature = "printing")] mod printing { use super :: * ; use attr :: FilterAttrs ; use data :: VariantData ; use quote :: { Tokens , ToTokens } ; impl ToTokens for DeriveInput { fn to_tokens (& self , tokens : & mut Tokens) { for attr in self . attrs . outer () { attr . to_tokens (tokens) ; } self . vis . to_tokens (tokens) ; match self . body { Body :: Enum (ref d) => d . enum_token . to_tokens (tokens) , Body :: Struct (ref d) => d . struct_token . to_tokens (tokens) , } self . ident . to_tokens (tokens) ; self . generics . to_tokens (tokens) ; match self . body { Body :: Enum (ref data) => { self . generics . where_clause . to_tokens (tokens) ; data . brace_token . surround (tokens , | tokens | { data . variants . to_tokens (tokens) ; }) ; } Body :: Struct (ref data) => { match data . data { VariantData :: Struct (..) => { self . generics . where_clause . to_tokens (tokens) ; data . data . to_tokens (tokens) ; } VariantData :: Tuple (..) => { data . data . to_tokens (tokens) ; self . generics . where_clause . to_tokens (tokens) ; } VariantData :: Unit => { self . generics . where_clause . to_tokens (tokens) ; } } data . semi_token . to_tokens (tokens) ; } } } } }
};
}
