// Generated macro for impl_21 (impl)
macro_rules! Depcrate_exportimpl_21 {
() => {
// Module: crate::export
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'tcx > AbiHashStable < 'tcx > for str { # [inline] fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { self . as_bytes () . abi_hash (tcx , hasher) ; } }
};
}
