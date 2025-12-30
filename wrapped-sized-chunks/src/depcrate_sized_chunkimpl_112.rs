// Generated macro for impl_112 (impl)
macro_rules! Depcrate_sized_chunkimpl_112 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_112"}
// Dependencies: {}
impl < 'a , A , const N : usize > Extend < & 'a A > for Chunk < A , N > where A : 'a + Copy , { # [doc = " Append the contents of the iterator to the back of the chunk."] # [doc = ""] # [doc = " Panics if the chunk exceeds its capacity."] # [doc = ""] # [doc = " Time: O(n) for the length of the iterator"] fn extend < I > (& mut self , it : I) where I : IntoIterator < Item = & 'a A > , { for item in it { self . push_back (* item) ; } } }
};
}
