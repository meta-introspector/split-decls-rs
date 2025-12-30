// Generated macro for impl_156 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_156 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_156"}
// Dependencies: {}
impl RustcInternal for AdtDef { type T < 'tcx > = rustc_ty :: AdtDef < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { InternalCx :: adt_def (tcx , self . 0 . internal (tables , tcx)) } }
};
}
