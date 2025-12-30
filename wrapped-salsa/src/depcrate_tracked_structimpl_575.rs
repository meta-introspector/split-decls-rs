// Generated macro for impl_575 (impl)
macro_rules! Depcrate_tracked_structimpl_575 {
() => {
// Module: crate::tracked_struct
// Provides: {"impl_575"}
// Dependencies: {}
impl < 'db , C > StructEntry < 'db , C > where C : Configuration , { # [doc = " Returns the `DatabaseKeyIndex` for this entry."] pub fn key (& self) -> DatabaseKeyIndex { self . key } # [doc = " Returns the tracked struct."] pub fn as_struct (& self) -> C :: Struct < '_ > { FromId :: from_id (self . key . key_index ()) } # [cfg (feature = "salsa_unstable")] pub fn value (& self) -> & 'db Value < C > { self . value } }
};
}
