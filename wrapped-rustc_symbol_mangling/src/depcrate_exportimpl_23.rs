// Generated macro for impl_23 (impl)
macro_rules! Depcrate_exportimpl_23 {
() => {
// Module: crate::export
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'tcx , T : AbiHashStable < 'tcx > > AbiHashStable < 'tcx > for [T] { fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { self . len () . abi_hash (tcx , hasher) ; for item in self { item . abi_hash (tcx , hasher) ; } } }
};
}
