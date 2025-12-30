// Generated macro for impl_160 (impl)
macro_rules! Depcrate_sparse_chunkimpl_160 {
() => {
// Module: crate::sparse_chunk
// Provides: {"impl_160"}
// Dependencies: {}
# [cfg (feature = "std")] impl < A , const N : usize > PartialEq < BTreeMap < usize , A > > for SparseChunk < A , N > where A : PartialEq , BitsImpl < N > : Bits , { fn eq (& self , other : & BTreeMap < usize , A >) -> bool { if self . len () != other . len () { return false ; } for index in self . indices () { if self . get (index) != other . get (& index) { return false ; } } true } }
};
}
