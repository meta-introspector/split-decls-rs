// Generated macro for impl_276 (impl)
macro_rules! Depcrate_inputimpl_276 {
() => {
// Module: crate::input
// Provides: {"impl_276"}
// Dependencies: {}
impl < 'db , C > StructEntry < 'db , C > where C : Configuration , { # [doc = " Returns the `DatabaseKeyIndex` for this entry."] pub fn key (& self) -> DatabaseKeyIndex { self . key } # [doc = " Returns the input struct."] pub fn as_struct (& self) -> C :: Struct { FromId :: from_id (self . key . key_index ()) } # [cfg (feature = "salsa_unstable")] pub fn value (& self) -> & 'db Value < C > { self . value } }
};
}
