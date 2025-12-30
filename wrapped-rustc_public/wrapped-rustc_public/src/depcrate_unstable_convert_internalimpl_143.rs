// Generated macro for impl_143 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_143 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_143"}
// Dependencies: {}
impl RustcInternal for MonoItem { type T < 'tcx > = rustc_middle :: mir :: mono :: MonoItem < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { use rustc_middle :: mir :: mono as rustc_mono ; match self { MonoItem :: Fn (instance) => rustc_mono :: MonoItem :: Fn (instance . internal (tables , tcx)) , MonoItem :: Static (def) => rustc_mono :: MonoItem :: Static (def . internal (tables , tcx)) , MonoItem :: GlobalAsm (_) => { unimplemented ! () } } } }
};
}
