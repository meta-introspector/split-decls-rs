// Generated macro for impl_25 (impl)
macro_rules! Depcrate_block_apiimpl_25 {
() => {
// Module: crate::block_api
// Provides: {"impl_25"}
// Dependencies: {}
impl UpdateCore for Sha512VarCore { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . block_len += blocks . len () as u128 ; let blocks = Array :: cast_slice_to_core (blocks) ; compress512 (& mut self . state , blocks) ; } }
};
}
