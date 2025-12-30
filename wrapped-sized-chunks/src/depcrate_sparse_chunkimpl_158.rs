// Generated macro for impl_158 (impl)
macro_rules! Depcrate_sparse_chunkimpl_158 {
() => {
// Module: crate::sparse_chunk
// Provides: {"impl_158"}
// Dependencies: {}
impl < A , const N : usize > FromIterator < Option < A > > for SparseChunk < A , N > where BitsImpl < N > : Bits , { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = Option < A > > , { let mut out = Self :: new () ; for (index , value) in iter . into_iter () . enumerate () { if let Some (value) = value { out . insert (index , value) ; } } out } }
};
}
