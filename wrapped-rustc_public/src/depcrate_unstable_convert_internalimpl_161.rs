// Generated macro for impl_161 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_161 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_161"}
// Dependencies: {}
impl RustcInternal for Place { type T < 'tcx > = rustc_middle :: mir :: Place < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { rustc_middle :: mir :: Place { local : rustc_middle :: mir :: Local :: from_usize (self . local) , projection : tcx . mk_place_elems (& self . projection . internal (tables , tcx)) , } } }
};
}
