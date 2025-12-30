// Generated macro for FromSerde (trait)
macro_rules! Depcrate_ast_from_serdeFromSerde {
() => {
// Module: crate::ast::from_serde
// Provides: {"FromSerde"}
// Dependencies: {}
pub trait FromSerde : Sized { type SerdeType ; fn from_serde (errors : & Ctxt , serde : Self :: SerdeType) -> Self ; fn vec_from_serde (errors : & Ctxt , serdes : Vec < Self :: SerdeType >) -> Vec < Self > { serdes . into_iter () . map (| s | Self :: from_serde (errors , s)) . collect () } }
};
}
