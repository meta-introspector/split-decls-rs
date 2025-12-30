// Generated macro for impl_325 (impl)
macro_rules! Depcrate_internedimpl_325 {
() => {
// Module: crate::interned
// Provides: {"impl_325"}
// Dependencies: {}
impl < 'db , C > StructEntry < 'db , C > where C : Configuration , { # [doc = " Returns the `DatabaseKeyIndex` for this entry."] pub fn key (& self) -> DatabaseKeyIndex { self . key } # [doc = " Returns the interned struct."] pub fn as_struct (& self) -> C :: Struct < '_ > { FromId :: from_id (self . key . key_index ()) } # [cfg (feature = "salsa_unstable")] pub fn value (& self) -> & 'db Value < C > { self . value } }
};
}
