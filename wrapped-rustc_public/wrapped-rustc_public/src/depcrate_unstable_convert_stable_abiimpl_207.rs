// Generated macro for impl_207 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_207 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_207"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: ReprFlags { type T = ReprFlags ; fn stable < 'cx > (& self , _tables : & mut Tables < 'cx , BridgeTys > , _cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { ReprFlags { is_simd : self . intersects (Self :: IS_SIMD) , is_c : self . intersects (Self :: IS_C) , is_transparent : self . intersects (Self :: IS_TRANSPARENT) , is_linear : self . intersects (Self :: IS_LINEAR) , } } }
};
}
