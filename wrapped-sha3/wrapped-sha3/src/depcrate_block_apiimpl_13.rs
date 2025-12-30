// Generated macro for impl_13 (impl)
macro_rules! Depcrate_block_apiimpl_13 {
() => {
// Module: crate::block_api
// Provides: {"impl_13"}
// Dependencies: {}
impl < Rate , OutputSize , const PAD : u8 , const ROUNDS : usize > FixedOutputCore for Sha3HasherCore < Rate , OutputSize , PAD , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , OutputSize : ArraySize + IsLessOrEqual < U200 , Output = True > , { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let pos = buffer . get_pos () ; let mut block = buffer . pad_with_zeros () ; block [pos] = PAD ; let n = block . len () ; block [n - 1] |= 0x80 ; xor_block (& mut self . state , & block) ; keccak :: p1600 (& mut self . state , ROUNDS) ; for (o , s) in out . chunks_mut (8) . zip (self . state . iter ()) { o . copy_from_slice (& s . to_le_bytes () [.. o . len ()]) ; } } }
};
}
