// Generated macro for impl_13 (impl)
macro_rules! Depcrate_ast_from_serdeimpl_13 {
() => {
// Module: crate::ast::from_serde
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a > FromSerde for Field < 'a > { type SerdeType = serde_ast :: Field < 'a > ; fn from_serde (errors : & Ctxt , serde : Self :: SerdeType) -> Self { Self { member : serde . member , serde_attrs : serde . attrs , ty : serde . ty , original : serde . original , attrs : FieldAttrs :: new (& serde . original . attrs , errors) , } } }
};
}
