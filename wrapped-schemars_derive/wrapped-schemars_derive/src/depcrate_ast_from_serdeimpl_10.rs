// Generated macro for impl_10 (impl)
macro_rules! Depcrate_ast_from_serdeimpl_10 {
() => {
// Module: crate::ast::from_serde
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'a > FromSerde for Container < 'a > { type SerdeType = serde_ast :: Container < 'a > ; fn from_serde (errors : & Ctxt , serde : Self :: SerdeType) -> Self { let data = Data :: from_serde (errors , serde . data) ; let attrs = ContainerAttrs :: new (& serde . original . attrs , & data , errors) ; let rename_type_params = match & attrs . rename_format_string { Some (s) => crate :: name :: get_rename_format_type_params (errors , s , serde . generics) , None => BTreeSet :: new () , } ; let mut cont = Self { ident : serde . ident , serde_attrs : serde . attrs , data , attrs , rename_type_params , relevant_type_params : BTreeSet :: new () , generics : serde . generics . clone () , } ; crate :: bound :: find_trait_bounds (serde . generics , & mut cont) ; cont } }
};
}
