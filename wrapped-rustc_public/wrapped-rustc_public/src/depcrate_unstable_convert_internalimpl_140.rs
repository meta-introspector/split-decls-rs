// Generated macro for impl_140 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_140 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_140"}
// Dependencies: {}
impl RustcInternal for VariantIdx { type T < 'tcx > = rustc_abi :: VariantIdx ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { rustc_abi :: VariantIdx :: from (self . to_index ()) } }
};
}
