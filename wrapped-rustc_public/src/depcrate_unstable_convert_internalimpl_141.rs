// Generated macro for impl_141 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_141 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_141"}
// Dependencies: {}
impl RustcInternal for VariantDef { type T < 'tcx > = & 'tcx rustc_ty :: VariantDef ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { self . adt_def . internal (tables , tcx) . variant (self . idx . internal (tables , tcx)) } }
};
}
