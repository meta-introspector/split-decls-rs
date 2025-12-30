// Generated macro for impl_11 (impl)
macro_rules! Depcrate_block_apiimpl_11 {
() => {
// Module: crate::block_api
// Provides: {"impl_11"}
// Dependencies: {}
impl UpdateCore for Sm3Core { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . block_len += blocks . len () as u64 ; compress (& mut self . h , Array :: cast_slice_to_core (blocks)) ; } }
};
}
