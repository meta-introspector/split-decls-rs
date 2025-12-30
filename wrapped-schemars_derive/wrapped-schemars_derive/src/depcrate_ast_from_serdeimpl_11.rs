// Generated macro for impl_11 (impl)
macro_rules! Depcrate_ast_from_serdeimpl_11 {
() => {
// Module: crate::ast::from_serde
// Provides: {"impl_11"}
// Dependencies: {}
impl < 'a > FromSerde for Data < 'a > { type SerdeType = serde_ast :: Data < 'a > ; fn from_serde (errors : & Ctxt , serde : Self :: SerdeType) -> Self { match serde { serde_ast :: Data :: Enum (variants) => { Data :: Enum (Variant :: vec_from_serde (errors , variants)) } serde_ast :: Data :: Struct (style , fields) => { Data :: Struct (style , Field :: vec_from_serde (errors , fields)) } } } }
};
}
