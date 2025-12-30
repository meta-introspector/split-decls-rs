// Generated macro for impl_146 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_146 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_146"}
// Dependencies: {}
# [allow (rustc :: usage_of_qualified_ty)] impl < T > RustcInternal for Binder < T > where T : RustcInternal , for < 'tcx > T :: T < 'tcx > : rustc_ty :: TypeVisitable < rustc_ty :: TyCtxt < 'tcx > > , { type T < 'tcx > = rustc_ty :: Binder < 'tcx , T :: T < 'tcx > > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { rustc_ty :: Binder :: bind_with_vars (self . value . internal (tables , tcx) , tcx . mk_bound_variable_kinds_from_iter (self . bound_vars . iter () . map (| bound | bound . internal (tables , tcx)) ,) ,) } }
};
}
