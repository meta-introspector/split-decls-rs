// Generated macro for impl_100 (impl)
macro_rules! Depcrate_sized_chunkimpl_100 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_100"}
// Dependencies: {}
impl < A , T , const N : usize > From < InlineArray < A , T > > for Chunk < A , N > { # [inline] fn from (mut array : InlineArray < A , T >) -> Self { Self :: from (& mut array) } }
};
}
