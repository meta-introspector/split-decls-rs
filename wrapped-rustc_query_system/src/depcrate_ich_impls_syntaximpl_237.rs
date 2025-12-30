// Generated macro for impl_237 (impl)
macro_rules! Depcrate_ich_impls_syntaximpl_237 {
() => {
// Module: crate::ich::impls_syntax
// Provides: {"impl_237"}
// Dependencies: {}
impl < 'tcx > HashStable < StableHashingContext < 'tcx > > for rustc_feature :: Features { fn hash_stable (& self , hcx : & mut StableHashingContext < 'tcx > , hasher : & mut StableHasher) { self . enabled_lang_features () . hash_stable (hcx , hasher) ; self . enabled_lib_features () . hash_stable (hcx , hasher) ; } }
};
}
