// Generated macro for TransmuteRefMutDstElemSizeNotMultiple (enum)
macro_rules! Depcrate_doctestsTransmuteRefMutDstElemSizeNotMultiple {
() => {
// Module: crate::doctests
// Provides: {"TransmuteRefMutDstElemSizeNotMultiple"}
// Dependencies: {}
# [doc = " Reference transmutes are not possible when the source's trailing slice"] # [doc = " element size is not a multiple of the destination's."] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " use zerocopy::doctests::SliceDst;"] # [doc = " let src: &SliceDst<(), [u8; 3]> = SliceDst::new();"] # [doc = " let _: &SliceDst<(), [u8; 2]> = zerocopy::transmute_ref!(src);"] # [doc = " ```"] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " use zerocopy::doctests::SliceDst;"] # [doc = " let src: &mut SliceDst<(), [u8; 3]> = SliceDst::new_mut();"] # [doc = " let _: &mut SliceDst<(), [u8; 2]> = zerocopy::transmute_mut!(src);"] # [doc = " ```"] enum TransmuteRefMutDstElemSizeNotMultiple { }
};
}
