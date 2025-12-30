// Generated macro for impl_239 (impl)
macro_rules! Depcrate_ich_impls_syntaximpl_239 {
() => {
// Module: crate::ich::impls_syntax
// Provides: {"impl_239"}
// Dependencies: {}
impl < 'tcx > HashStable < StableHashingContext < 'tcx > > for rustc_feature :: EnabledLibFeature { fn hash_stable (& self , hcx : & mut StableHashingContext < 'tcx > , hasher : & mut StableHasher) { let rustc_feature :: EnabledLibFeature { gate_name , attr_sp } = self ; gate_name . hash_stable (hcx , hasher) ; attr_sp . hash_stable (hcx , hasher) ; } }
};
}
