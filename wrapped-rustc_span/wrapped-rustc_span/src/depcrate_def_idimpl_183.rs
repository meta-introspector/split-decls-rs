// Generated macro for impl_183 (impl)
macro_rules! Depcrate_def_idimpl_183 {
() => {
// Module: crate::def_id
// Provides: {"impl_183"}
// Dependencies: {}
impl < CTX : HashStableContext > HashStable < CTX > for DefId { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { hcx . def_path_hash (* self) . hash_stable (hcx , hasher) ; } }
};
}
