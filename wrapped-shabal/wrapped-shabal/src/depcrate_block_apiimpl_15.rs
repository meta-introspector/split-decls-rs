// Generated macro for impl_15 (impl)
macro_rules! Depcrate_block_apiimpl_15 {
() => {
// Module: crate::block_api
// Provides: {"impl_15"}
// Dependencies: {}
impl UpdateCore for ShabalVarCore { # [inline] fn update_blocks (& mut self , blocks : & [Block]) { for block in blocks { let m = read_m (block) ; self . add_m (& m) ; self . xor_w () ; self . perm (& m) ; self . sub_m (& m) ; self . swap_b_c () ; self . w += Wrapping (1) ; } } }
};
}
