// Generated macro for impl_25 (impl)
macro_rules! Depcrate_exportimpl_25 {
() => {
// Module: crate::export
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'tcx > AbiHashStable < 'tcx > for ty :: FnSig < 'tcx > { fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { for ty in self . inputs_and_output { ty . abi_hash (tcx , hasher) ; } self . safety . is_safe () . abi_hash (tcx , hasher) ; } }
};
}
