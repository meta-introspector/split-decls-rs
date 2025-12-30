// Generated macro for impl_90 (impl)
macro_rules! Depcrate_sized_chunkimpl_90 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_90"}
// Dependencies: {}
impl < A , I , const N : usize > Index < I > for Chunk < A , N > where I : SliceIndex < [A] > , { type Output = I :: Output ; fn index (& self , index : I) -> & Self :: Output { self . as_slice () . index (index) } }
};
}
