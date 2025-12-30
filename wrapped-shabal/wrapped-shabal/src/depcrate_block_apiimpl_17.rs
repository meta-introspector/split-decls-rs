// Generated macro for impl_17 (impl)
macro_rules! Depcrate_block_apiimpl_17 {
() => {
// Module: crate::block_api
// Provides: {"impl_17"}
// Dependencies: {}
impl VariableOutputCore for ShabalVarCore { const TRUNC_SIDE : TruncSide = TruncSide :: Right ; # [inline] # [allow (clippy :: needless_range_loop)] fn new (output_size : usize) -> Result < Self , InvalidOutputSize > { let init = match output_size { 24 => consts :: INIT_192 , 28 => consts :: INIT_224 , 32 => consts :: INIT_256 , 48 => consts :: INIT_384 , 64 => consts :: INIT_512 , _ => return Err (InvalidOutputSize) , } ; Ok (Self { a : init . 0 . map (Wrapping) , b : init . 1 . map (Wrapping) , c : init . 2 . map (Wrapping) , w : Wrapping (1) , }) } # [inline] fn finalize_variable_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let pos = buffer . get_pos () ; let mut block = buffer . pad_with_zeros () ; block [pos] = 0x80 ; let m = read_m (& block) ; self . add_m (& m) ; self . xor_w () ; self . perm (& m) ; for _ in 0 .. 3 { self . swap_b_c () ; self . xor_w () ; self . perm (& m) ; } for (chunk , v) in out . chunks_exact_mut (4) . zip (self . b . iter ()) { chunk . copy_from_slice (& v . 0 . to_le_bytes ()) ; } } }
};
}
