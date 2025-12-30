// Generated macro for derive_parse (function)
macro_rules! Depcrate_parsederive_parse {
() => {
// Module: crate::parse
// Provides: {"derive_parse"}
// Dependencies: {}
pub fn derive_parse (input : DeriveInput) -> Result < TokenStream > { let mut dump = false ; for attr in & input . attrs { if attr . path () . is_ident ("parse") { let attr : ParseAttribute = attr . parse_args () ? ; dump = dump || attr . dump . is_some () ; } } let ts = match & input . data { Data :: Struct (data) => code_from_struct (data) ? , Data :: Enum (data) => code_from_enum (& input . ident , data) ? , Data :: Union (_) => { bail ! (Span :: call_site () , "Not supported for union.") } } ; let ts = quote ! { fn parse (input : :: structmeta :: helpers :: exports :: syn :: parse :: ParseStream <'_ >) -> :: structmeta :: helpers :: exports :: syn :: Result < Self > { # ts } } ; let ts = impl_trait_result (& input , & parse_quote ! (:: structmeta :: helpers :: exports :: syn :: parse :: Parse) , & [] , ts , dump ,) ? ; Ok (ts) }
};
}
