// Generated macro for impl_15 (impl)
macro_rules! Depcrate_block_apiimpl_15 {
() => {
// Module: crate::block_api
// Provides: {"impl_15"}
// Dependencies: {}
impl FixedOutputCore for Sha1Core { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let bs = Self :: BlockSize :: U64 ; let bit_len = 8 * (buffer . get_pos () as u64 + bs * self . block_len) ; let mut h = self . h ; buffer . len64_padding_be (bit_len , | b | compress (& mut h , & [b . 0])) ; for (chunk , v) in out . chunks_exact_mut (4) . zip (h . iter ()) { chunk . copy_from_slice (& v . to_be_bytes ()) ; } } }
};
}
