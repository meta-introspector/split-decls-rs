// Generated macro for impl_184 (impl)
macro_rules! Depcrate_def_idimpl_184 {
() => {
// Module: crate::def_id
// Provides: {"impl_184"}
// Dependencies: {}
impl < CTX : HashStableContext > HashStable < CTX > for LocalDefId { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { hcx . def_path_hash (self . to_def_id ()) . local_hash () . hash_stable (hcx , hasher) ; } }
};
}
