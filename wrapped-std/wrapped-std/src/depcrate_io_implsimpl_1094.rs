// Generated macro for impl_1094 (impl)
macro_rules! Depcrate_io_implsimpl_1094 {
() => {
// Module: crate::io::impls
// Provides: {"impl_1094"}
// Dependencies: {}
# [doc = " BufRead is implemented for `VecDeque<u8>` by reading bytes from the front of the `VecDeque`."] # [stable (feature = "vecdeque_buf_read" , since = "1.75.0")] impl < A : Allocator > BufRead for VecDeque < u8 , A > { # [doc = " Returns the contents of the \"front\" slice as returned by"] # [doc = " [`as_slices`][`VecDeque::as_slices`]. If the contained byte slices of the `VecDeque` are"] # [doc = " discontiguous, multiple calls to `fill_buf` will be needed to read the entire content."] # [inline] fn fill_buf (& mut self) -> io :: Result < & [u8] > { let (front , _) = self . as_slices () ; Ok (front) } # [inline] fn consume (& mut self , amt : usize) { self . drain (.. amt) ; } }
};
}
