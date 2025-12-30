// Generated macro for impl_223 (impl)
macro_rules! Depcrate_ich_hcximpl_223 {
() => {
// Module: crate::ich::hcx
// Provides: {"impl_223"}
// Dependencies: {}
impl < 'a > HashStable < StableHashingContext < 'a > > for ast :: NodeId { # [inline] fn hash_stable (& self , _ : & mut StableHashingContext < 'a > , _ : & mut StableHasher) { panic ! ("Node IDs should not appear in incremental state") ; } }
};
}
