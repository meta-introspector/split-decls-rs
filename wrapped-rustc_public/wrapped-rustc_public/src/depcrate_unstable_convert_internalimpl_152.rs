// Generated macro for impl_152 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_152 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_152"}
// Dependencies: {}
impl RustcInternal for ExistentialTraitRef { type T < 'tcx > = rustc_ty :: ExistentialTraitRef < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { use crate :: unstable :: internal_cx :: ExistentialTraitRefHelpers ; tcx . new_from_args (self . def_id . 0 . internal (tables , tcx) , self . generic_args . internal (tables , tcx) ,) } }
};
}
