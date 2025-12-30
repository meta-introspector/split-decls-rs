// Generated macro for impl_153 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_153 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_153"}
// Dependencies: {}
impl RustcInternal for TraitRef { type T < 'tcx > = rustc_ty :: TraitRef < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { use crate :: unstable :: internal_cx :: TraitRefHelpers ; tcx . new_from_args (self . def_id . 0 . internal (tables , tcx) , self . args () . internal (tables , tcx)) } }
};
}
