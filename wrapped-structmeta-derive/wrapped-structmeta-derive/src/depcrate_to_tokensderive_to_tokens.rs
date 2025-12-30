// Generated macro for derive_to_tokens (function)
macro_rules! Depcrate_to_tokensderive_to_tokens {
() => {
// Module: crate::to_tokens
// Provides: {"derive_to_tokens"}
// Dependencies: {}
pub fn derive_to_tokens (input : DeriveInput) -> Result < TokenStream > { let mut dump = false ; for attr in & input . attrs { if attr . path () . is_ident ("to_tokens") { let attr : ToTokensAttribute = attr . parse_args () ? ; dump = dump || attr . dump . is_some () ; } } let ts = match & input . data { Data :: Struct (data) => code_from_struct (data) ? , Data :: Enum (data) => code_from_enum (data) ? , Data :: Union (_) => { bail ! (Span :: call_site () , "Not supported for union.") } } ; let ts = quote ! { fn to_tokens (& self , tokens : & mut :: structmeta :: helpers :: exports :: proc_macro2 :: TokenStream) { # ts } } ; let ts = impl_trait_result (& input , & parse_quote ! (:: structmeta :: helpers :: exports :: quote :: ToTokens) , & [] , ts , dump ,) ? ; Ok (ts) }
};
}
