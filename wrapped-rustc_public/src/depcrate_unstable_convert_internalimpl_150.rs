// Generated macro for impl_150 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_150 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_150"}
// Dependencies: {}
impl RustcInternal for ExistentialProjection { type T < 'tcx > = rustc_ty :: ExistentialProjection < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { use crate :: unstable :: internal_cx :: ExistentialProjectionHelpers ; tcx . new_from_args (self . def_id . 0 . internal (tables , tcx) , self . generic_args . internal (tables , tcx) , self . term . internal (tables , tcx) ,) } }
};
}
