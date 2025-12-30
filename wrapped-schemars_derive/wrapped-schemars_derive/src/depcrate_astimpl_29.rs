// Generated macro for impl_29 (impl)
macro_rules! Depcrate_astimpl_29 {
() => {
// Module: crate::ast
// Provides: {"impl_29"}
// Dependencies: {}
impl quote :: ToTokens for Name < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let ser_name = self . 0 . serialize_name () ; let de_name = self . 0 . deserialize_name () ; if ser_name == de_name { ser_name . to_tokens (tokens) ; } else { quote ! { if # GENERATOR . contract () . is_serialize () { # ser_name } else { # de_name } } . to_tokens (tokens) ; } } }
};
}
