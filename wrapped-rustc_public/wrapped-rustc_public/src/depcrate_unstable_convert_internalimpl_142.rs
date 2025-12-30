// Generated macro for impl_142 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_142 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_142"}
// Dependencies: {}
impl RustcInternal for MirConst { type T < 'tcx > = rustc_middle :: mir :: Const < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { let constant = tables . mir_consts [self . id] ; match constant { rustc_middle :: mir :: Const :: Ty (ty , ct) => { rustc_middle :: mir :: Const :: Ty (tcx . lift (ty) . unwrap () , tcx . lift (ct) . unwrap ()) } rustc_middle :: mir :: Const :: Unevaluated (uneval , ty) => { rustc_middle :: mir :: Const :: Unevaluated (tcx . lift (uneval) . unwrap () , tcx . lift (ty) . unwrap () ,) } rustc_middle :: mir :: Const :: Val (const_val , ty) => { rustc_middle :: mir :: Const :: Val (tcx . lift (const_val) . unwrap () , tcx . lift (ty) . unwrap ()) } } } }
};
}
