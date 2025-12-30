// Generated macro for impl_185 (impl)
macro_rules! Depcrate_def_idimpl_185 {
() => {
// Module: crate::def_id
// Provides: {"impl_185"}
// Dependencies: {}
impl < CTX : HashStableContext > HashStable < CTX > for CrateNum { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . as_def_id () . to_stable_hash_key (hcx) . stable_crate_id () . hash_stable (hcx , hasher) ; } }
};
}
