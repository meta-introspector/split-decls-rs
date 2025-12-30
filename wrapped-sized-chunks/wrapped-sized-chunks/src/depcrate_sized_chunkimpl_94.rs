// Generated macro for impl_94 (impl)
macro_rules! Depcrate_sized_chunkimpl_94 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_94"}
// Dependencies: {}
impl < A , Slice , const N : usize > PartialEq < Slice > for Chunk < A , N > where Slice : Borrow < [A] > , A : PartialEq , { fn eq (& self , other : & Slice) -> bool { self . as_slice () == other . borrow () } }
};
}
