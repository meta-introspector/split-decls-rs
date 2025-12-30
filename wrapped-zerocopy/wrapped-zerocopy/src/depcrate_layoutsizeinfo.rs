// Generated macro for SizeInfo (enum)
macro_rules! Depcrate_layoutSizeInfo {
() => {
// Module: crate::layout
// Provides: {"SizeInfo"}
// Dependencies: {}
# [cfg_attr (any (kani , test) , derive (Debug , PartialEq , Eq))] # [derive (Copy , Clone)] pub (crate) enum SizeInfo < E = usize > { Sized { size : usize } , SliceDst (TrailingSliceLayout < E >) , }
};
}
