// Generated macro for impl_277 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_277 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_277"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: adjustment :: PointerCoercion { type T = crate :: mir :: PointerCoercion ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: ty :: adjustment :: PointerCoercion ; match self { PointerCoercion :: ReifyFnPointer => crate :: mir :: PointerCoercion :: ReifyFnPointer , PointerCoercion :: UnsafeFnPointer => crate :: mir :: PointerCoercion :: UnsafeFnPointer , PointerCoercion :: ClosureFnPointer (safety) => { crate :: mir :: PointerCoercion :: ClosureFnPointer (safety . stable (tables , cx)) } PointerCoercion :: MutToConstPointer => crate :: mir :: PointerCoercion :: MutToConstPointer , PointerCoercion :: ArrayToPointer => crate :: mir :: PointerCoercion :: ArrayToPointer , PointerCoercion :: Unsize => crate :: mir :: PointerCoercion :: Unsize , } } }
};
}
