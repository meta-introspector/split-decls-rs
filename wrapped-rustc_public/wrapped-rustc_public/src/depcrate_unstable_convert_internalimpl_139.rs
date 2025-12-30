// Generated macro for impl_139 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_139 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_139"}
// Dependencies: {}
impl RustcInternal for FnSig { type T < 'tcx > = rustc_ty :: FnSig < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (rustc_ty :: FnSig { inputs_and_output : tcx . mk_type_list (& self . inputs_and_output . internal (tables , tcx)) , c_variadic : self . c_variadic , safety : self . safety . internal (tables , tcx) , abi : self . abi . internal (tables , tcx) , }) . unwrap () } }
};
}
