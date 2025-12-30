// Generated macro for AsMap (trait)
macro_rules! Depcrate_fieldsAsMap {
() => {
// Module: crate::fields
// Provides: {"AsMap"}
// Dependencies: {}
pub trait AsMap : Sized + sealed :: Sealed { fn field_map (& self) -> SerializeFieldMap < '_ , Self > { SerializeFieldMap (self) } }
};
}
