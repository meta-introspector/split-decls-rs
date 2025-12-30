// Generated macro for impl_187 (impl)
macro_rules! Depcrate_def_idimpl_187 {
() => {
// Module: crate::def_id
// Provides: {"impl_187"}
// Dependencies: {}
impl < CTX : HashStableContext > ToStableHashKey < CTX > for LocalDefId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { hcx . def_path_hash (self . to_def_id ()) } }
};
}
