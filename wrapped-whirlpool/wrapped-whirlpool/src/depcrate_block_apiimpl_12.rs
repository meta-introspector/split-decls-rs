// Generated macro for impl_12 (impl)
macro_rules! Depcrate_block_apiimpl_12 {
() => {
// Module: crate::block_api
// Provides: {"impl_12"}
// Dependencies: {}
impl UpdateCore for WhirlpoolCore { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . blocks_len += blocks . len () as u64 ; let blocks = Array :: cast_slice_to_core (blocks) ; compress (& mut self . state , blocks) ; } }
};
}
