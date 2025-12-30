// Generated macro for derive_struct_meta (function)
macro_rules! Depcrate_struct_metaderive_struct_meta {
() => {
// Module: crate::struct_meta
// Provides: {"derive_struct_meta"}
// Dependencies: {}
pub fn derive_struct_meta (input : DeriveInput) -> Result < TokenStream > { if let Data :: Struct (data) = & input . data { let mut args = ArgsForStruct :: default () ; for attr in & input . attrs { if attr . path () . is_ident ("struct_meta") { args . parse_from_attr (attr) ? ; } } let ps = Params :: from_fields (& data . fields , & args) ? ; let body = ps . build () ; impl_trait_result (& input , & parse_quote ! (:: structmeta :: helpers :: exports :: syn :: parse :: Parse) , & [] , quote ! { fn parse (input : :: structmeta :: helpers :: exports :: syn :: parse :: ParseStream <'_ >) -> :: structmeta :: helpers :: exports :: syn :: Result < Self > { # body } } , args . dump ,) } else { let span = input . span () ; bail ! (span , "`#[derive(StructMeta)]` supports only struct.") } }
};
}
