// Generated macro for impl_126 (impl)
macro_rules! Depcrate_byte_sliceimpl_126 {
() => {
// Module: crate::byte_slice
// Provides: {"impl_126"}
// Dependencies: {}
unsafe impl SplitByteSlice for cell :: RefMut < '_ , [u8] > { # [inline] unsafe fn split_at_unchecked (self , mid : usize) -> (Self , Self) { cell :: RefMut :: map_split (self , | slice | unsafe { SplitByteSlice :: split_at_unchecked (slice , mid) }) } }
};
}
