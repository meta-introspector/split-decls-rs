// Generated macro for impl_245 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_245 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_245"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: NonDivergingIntrinsic < 'tcx > { type T = crate :: mir :: NonDivergingIntrinsic ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: mir :: NonDivergingIntrinsic ; use crate :: mir :: CopyNonOverlapping ; match self { NonDivergingIntrinsic :: Assume (op) => { crate :: mir :: NonDivergingIntrinsic :: Assume (op . stable (tables , cx)) } NonDivergingIntrinsic :: CopyNonOverlapping (copy_non_overlapping) => { crate :: mir :: NonDivergingIntrinsic :: CopyNonOverlapping (CopyNonOverlapping { src : copy_non_overlapping . src . stable (tables , cx) , dst : copy_non_overlapping . dst . stable (tables , cx) , count : copy_non_overlapping . count . stable (tables , cx) , }) } } } }
};
}
