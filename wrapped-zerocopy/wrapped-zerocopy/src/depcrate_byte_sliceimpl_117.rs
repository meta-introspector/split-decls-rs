// Generated macro for impl_117 (impl)
macro_rules! Depcrate_byte_sliceimpl_117 {
() => {
// Module: crate::byte_slice
// Provides: {"impl_117"}
// Dependencies: {}
unsafe impl SplitByteSlice for & [u8] { # [inline] unsafe fn split_at_unchecked (self , mid : usize) -> (Self , Self) { unsafe { (< [u8] > :: get_unchecked (self , .. mid) , < [u8] > :: get_unchecked (self , mid ..)) } } }
};
}
