// Generated macro for impl_20 (impl)
macro_rules! Depcrate_exportimpl_20 {
() => {
// Module: crate::export
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'tcx > AbiHashStable < 'tcx > for bool { # [inline] fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { (if * self { 1u8 } else { 0u8 }) . abi_hash (tcx , hasher) ; } }
};
}
