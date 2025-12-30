// Generated macro for impl_108 (impl)
macro_rules! Depcrate_sized_chunkimpl_108 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_108"}
// Dependencies: {}
impl < A , const N : usize > FromIterator < A > for Chunk < A , N > { fn from_iter < I > (it : I) -> Self where I : IntoIterator < Item = A > , { let mut chunk = Self :: new () ; for item in it { chunk . push_back (item) ; } chunk } }
};
}
