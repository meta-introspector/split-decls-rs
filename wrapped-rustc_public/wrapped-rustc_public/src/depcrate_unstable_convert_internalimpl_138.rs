// Generated macro for impl_138 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_138 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_138"}
// Dependencies: {}
impl RustcInternal for RawPtrKind { type T < 'tcx > = rustc_middle :: mir :: RawPtrKind ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { RawPtrKind :: Mut => rustc_middle :: mir :: RawPtrKind :: Mut , RawPtrKind :: Const => rustc_middle :: mir :: RawPtrKind :: Const , RawPtrKind :: FakeForPtrMetadata => rustc_middle :: mir :: RawPtrKind :: FakeForPtrMetadata , } } }
};
}
