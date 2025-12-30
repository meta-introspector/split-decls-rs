// Generated macro for impl_26 (impl)
macro_rules! Depcrate_exportimpl_26 {
() => {
// Module: crate::export
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'tcx > AbiHashStable < 'tcx > for ty :: GenericArg < 'tcx > { fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { self . kind () . abi_hash (tcx , hasher) ; } }
};
}
