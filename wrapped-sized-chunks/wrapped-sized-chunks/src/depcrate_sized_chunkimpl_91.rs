// Generated macro for impl_91 (impl)
macro_rules! Depcrate_sized_chunkimpl_91 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_91"}
// Dependencies: {}
impl < A , I , const N : usize > IndexMut < I > for Chunk < A , N > where I : SliceIndex < [A] > , { fn index_mut (& mut self , index : I) -> & mut Self :: Output { self . as_mut_slice () . index_mut (index) } }
};
}
