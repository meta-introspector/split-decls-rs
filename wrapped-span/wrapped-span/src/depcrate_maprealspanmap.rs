// Generated macro for RealSpanMap (struct)
macro_rules! Depcrate_mapRealSpanMap {
() => {
// Module: crate::map
// Provides: {"RealSpanMap"}
// Dependencies: {}
# [derive (PartialEq , Eq , Hash , Debug)] pub struct RealSpanMap { file_id : EditionedFileId , # [doc = " Invariant: Sorted vec over TextSize"] pairs : Box < [(TextSize , ErasedFileAstId)] > , end : TextSize , }
};
}
