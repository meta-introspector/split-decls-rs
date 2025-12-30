// Generated macro for impl_137 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_137 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_137"}
// Dependencies: {}
impl RustcInternal for Movability { type T < 'tcx > = rustc_ty :: Movability ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { Movability :: Static => rustc_ty :: Movability :: Static , Movability :: Movable => rustc_ty :: Movability :: Movable , } } }
};
}
