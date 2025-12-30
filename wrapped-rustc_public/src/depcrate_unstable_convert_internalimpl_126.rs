// Generated macro for impl_126 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_126 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_126"}
// Dependencies: {}
impl RustcInternal for GenericArgs { type T < 'tcx > = rustc_ty :: GenericArgsRef < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { InternalCx :: mk_args_from_iter (tcx , self . 0 . iter () . map (| arg | arg . internal (tables , tcx))) } }
};
}
