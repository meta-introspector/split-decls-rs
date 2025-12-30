// Generated macro for impl_15 (impl)
macro_rules! Depcrate_block_apiimpl_15 {
() => {
// Module: crate::block_api
// Provides: {"impl_15"}
// Dependencies: {}
impl < const V2 : bool > FixedOutputCore for TigerCore < V2 > { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let bs = Self :: BlockSize :: U64 ; let pos = buffer . get_pos () as u64 ; let bit_len = 8 * (pos + bs * self . block_len) ; if V2 { buffer . len64_padding_le (bit_len , | b | compress (& mut self . state , b . as_ref ())) ; } else { buffer . digest_pad (1 , & bit_len . to_le_bytes () , | b | { compress (& mut self . state , b . as_ref ()) }) ; } for (chunk , v) in out . chunks_exact_mut (8) . zip (self . state . iter ()) { chunk . copy_from_slice (& v . to_le_bytes ()) ; } } }
};
}
