// Generated macro for impl_22 (impl)
macro_rules! Depcrate_exportimpl_22 {
() => {
// Module: crate::export
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'tcx > AbiHashStable < 'tcx > for Symbol { # [inline] fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { self . as_str () . abi_hash (tcx , hasher) ; } }
};
}
