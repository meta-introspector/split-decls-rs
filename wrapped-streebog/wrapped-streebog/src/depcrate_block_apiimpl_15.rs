// Generated macro for impl_15 (impl)
macro_rules! Depcrate_block_apiimpl_15 {
() => {
// Module: crate::block_api
// Provides: {"impl_15"}
// Dependencies: {}
impl UpdateCore for StreebogVarCore { # [inline] fn update_blocks (& mut self , blocks : & [GenBlock < Self >]) { for block in blocks { self . compress (block . as_ref () , BLOCK_SIZE as u64) ; } } }
};
}
