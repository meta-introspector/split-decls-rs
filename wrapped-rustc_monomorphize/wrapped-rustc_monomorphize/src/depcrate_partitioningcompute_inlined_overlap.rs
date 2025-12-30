// Generated macro for compute_inlined_overlap (function)
macro_rules! Depcrate_partitioningcompute_inlined_overlap {
() => {
// Module: crate::partitioning
// Provides: {"compute_inlined_overlap"}
// Dependencies: {}
# [doc = " Compute the combined size of all inlined items that appear in both `cgu1`"] # [doc = " and `cgu2`."] fn compute_inlined_overlap < 'tcx > (cgu1 : & CodegenUnit < 'tcx > , cgu2 : & CodegenUnit < 'tcx >) -> usize { let (src_cgu , dst_cgu) = if cgu1 . items () . len () <= cgu2 . items () . len () { (cgu1 , cgu2) } else { (cgu2 , cgu1) } ; let mut overlap = 0 ; for (item , data) in src_cgu . items () . iter () { if data . inlined && dst_cgu . items () . contains_key (item) { overlap += data . size_estimate ; } } overlap }
};
}
