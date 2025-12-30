// Generated macro for to_tokens (function)
macro_rules! Depcrate_attr_schemars_to_serdeto_tokens {
() => {
// Module: crate::attr::schemars_to_serde
// Provides: {"to_tokens"}
// Dependencies: {}
fn to_tokens (attrs : & [Attribute]) -> impl ToTokens { let mut tokens = proc_macro2 :: TokenStream :: new () ; for attr in attrs { attr . to_tokens (& mut tokens) ; } tokens }
};
}
