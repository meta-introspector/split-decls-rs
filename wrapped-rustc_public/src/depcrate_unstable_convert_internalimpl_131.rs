// Generated macro for impl_131 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_131 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_131"}
// Dependencies: {}
impl RustcInternal for Pattern { type T < 'tcx > = rustc_ty :: Pattern < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . mk_pat (match self { Pattern :: Range { start , end , include_end : _ } => rustc_ty :: PatternKind :: Range { start : start . as_ref () . unwrap () . internal (tables , tcx) , end : end . as_ref () . unwrap () . internal (tables , tcx) , } , }) } }
};
}
