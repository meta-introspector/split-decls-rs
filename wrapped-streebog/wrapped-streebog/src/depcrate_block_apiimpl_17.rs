// Generated macro for impl_17 (impl)
macro_rules! Depcrate_block_apiimpl_17 {
() => {
// Module: crate::block_api
// Provides: {"impl_17"}
// Dependencies: {}
impl VariableOutputCore for StreebogVarCore { const TRUNC_SIDE : TruncSide = TruncSide :: Right ; # [inline] fn new (output_size : usize) -> Result < Self , InvalidOutputSize > { let h = match output_size { 32 => [0x0101_0101_0101_0101 ; 8] , 64 => [0 ; 8] , _ => return Err (InvalidOutputSize) , } ; let (n , sigma) = Default :: default () ; Ok (Self { h , n , sigma }) } # [inline] fn finalize_variable_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let pos = buffer . get_pos () ; let mut block = buffer . pad_with_zeros () ; block [pos] = 1 ; self . compress (block . as_ref () , pos as u64) ; g (& mut self . h , & [0u64 ; 8] , & self . n) ; g (& mut self . h , & [0u64 ; 8] , & self . sigma) ; out . copy_from_slice (& to_bytes (& self . h)) ; } }
};
}
