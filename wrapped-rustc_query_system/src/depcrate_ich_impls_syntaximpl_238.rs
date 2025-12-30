// Generated macro for impl_238 (impl)
macro_rules! Depcrate_ich_impls_syntaximpl_238 {
() => {
// Module: crate::ich::impls_syntax
// Provides: {"impl_238"}
// Dependencies: {}
impl < 'tcx > HashStable < StableHashingContext < 'tcx > > for rustc_feature :: EnabledLangFeature { fn hash_stable (& self , hcx : & mut StableHashingContext < 'tcx > , hasher : & mut StableHasher) { let rustc_feature :: EnabledLangFeature { gate_name , attr_sp , stable_since } = self ; gate_name . hash_stable (hcx , hasher) ; attr_sp . hash_stable (hcx , hasher) ; stable_since . hash_stable (hcx , hasher) ; } }
};
}
