// Generated macro for impl_21 (impl)
macro_rules! Depcrate_xsalsaimpl_21 {
() => {
// Module: crate::xsalsa
// Provides: {"impl_21"}
// Dependencies: {}
impl < R : Unsigned > StreamCipherSeekCore for XSalsaCore < R > { type Counter = u64 ; # [inline (always)] fn get_block_pos (& self) -> u64 { self . 0 . get_block_pos () } # [inline (always)] fn set_block_pos (& mut self , pos : u64) { self . 0 . set_block_pos (pos) ; } }
};
}
