// Generated macro for impl_13 (impl)
macro_rules! Depcrate_block_apiimpl_13 {
() => {
// Module: crate::block_api
// Provides: {"impl_13"}
// Dependencies: {}
impl FixedOutputCore for WhirlpoolCore { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let pos = buffer . get_pos () ; let block_size = Self :: block_size () as u128 ; let byte_len = block_size * (self . blocks_len as u128) + (pos as u128) ; let bit_len = 8 * byte_len ; let mut buf = [0u8 ; 32] ; buf [16 ..] . copy_from_slice (& bit_len . to_be_bytes ()) ; let mut state = self . state ; buffer . digest_pad (0x80 , & buf , | block | { compress (& mut state , & [block . 0]) ; }) ; for (chunk , v) in out . chunks_exact_mut (8) . zip (state . iter ()) { chunk . copy_from_slice (& v . to_le_bytes ()) ; } } }
};
}
