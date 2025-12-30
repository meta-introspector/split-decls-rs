// Generated macro for impl_14 (impl)
macro_rules! Depcrate_block_apiimpl_14 {
() => {
// Module: crate::block_api
// Provides: {"impl_14"}
// Dependencies: {}
impl UpdateCore for Sha1Core { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . block_len += blocks . len () as u64 ; let blocks = Array :: cast_slice_to_core (blocks) ; compress (& mut self . h , blocks) ; } }
};
}
