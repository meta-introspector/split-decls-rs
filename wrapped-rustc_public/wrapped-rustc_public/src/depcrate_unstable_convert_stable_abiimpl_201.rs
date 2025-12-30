// Generated macro for impl_201 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_201 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_201"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: Scalar { type T = Scalar ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { rustc_abi :: Scalar :: Initialized { value , valid_range } => Scalar :: Initialized { value : value . stable (tables , cx) , valid_range : valid_range . stable (tables , cx) , } , rustc_abi :: Scalar :: Union { value } => Scalar :: Union { value : value . stable (tables , cx) } , } } }
};
}
