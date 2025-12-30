// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl < R : Unsigned > StreamCipherSeekCore for SalsaCore < R > { type Counter = u64 ; # [inline (always)] fn get_block_pos (& self) -> u64 { (self . state [8] as u64) + ((self . state [9] as u64) << 32) } # [inline (always)] fn set_block_pos (& mut self , pos : u64) { self . state [8] = (pos & 0xffff_ffff) as u32 ; self . state [9] = ((pos >> 32) & 0xffff_ffff) as u32 ; } }
};
}
