// Generated macro for impl_12 (impl)
macro_rules! Depcrate_ast_from_serdeimpl_12 {
() => {
// Module: crate::ast::from_serde
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a > FromSerde for Variant < 'a > { type SerdeType = serde_ast :: Variant < 'a > ; fn from_serde (errors : & Ctxt , serde : Self :: SerdeType) -> Self { Self { ident : serde . ident , serde_attrs : serde . attrs , style : serde . style , fields : Field :: vec_from_serde (errors , serde . fields) , original : serde . original , attrs : VariantAttrs :: new (& serde . original . attrs , errors) , } } }
};
}
