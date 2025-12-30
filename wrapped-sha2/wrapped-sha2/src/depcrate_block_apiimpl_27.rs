// Generated macro for impl_27 (impl)
macro_rules! Depcrate_block_apiimpl_27 {
() => {
// Module: crate::block_api
// Provides: {"impl_27"}
// Dependencies: {}
impl VariableOutputCore for Sha512VarCore { const TRUNC_SIDE : TruncSide = TruncSide :: Left ; # [inline] fn new (output_size : usize) -> Result < Self , InvalidOutputSize > { let state = match output_size { 28 => consts :: H512_224 , 32 => consts :: H512_256 , 48 => consts :: H512_384 , 64 => consts :: H512_512 , _ => return Err (InvalidOutputSize) , } ; let block_len = 0 ; Ok (Self { state , block_len }) } # [inline] fn finalize_variable_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let bs = Self :: BlockSize :: U64 as u128 ; let bit_len = 8 * (buffer . get_pos () as u128 + bs * self . block_len) ; buffer . len128_padding_be (bit_len , | b | compress512 (& mut self . state , & [b . 0])) ; for (chunk , v) in out . chunks_exact_mut (8) . zip (self . state . iter ()) { chunk . copy_from_slice (& v . to_be_bytes ()) ; } } }
};
}
