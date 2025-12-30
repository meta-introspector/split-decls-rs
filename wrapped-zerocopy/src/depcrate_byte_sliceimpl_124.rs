// Generated macro for impl_124 (impl)
macro_rules! Depcrate_byte_sliceimpl_124 {
() => {
// Module: crate::byte_slice
// Provides: {"impl_124"}
// Dependencies: {}
unsafe impl SplitByteSlice for cell :: Ref < '_ , [u8] > { # [inline] unsafe fn split_at_unchecked (self , mid : usize) -> (Self , Self) { cell :: Ref :: map_split (self , | slice | unsafe { SplitByteSlice :: split_at_unchecked (slice , mid) }) } }
};
}
