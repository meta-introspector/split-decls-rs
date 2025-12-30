// Generated macro for impl_14 (impl)
macro_rules! Depcrate_block_apiimpl_14 {
() => {
// Module: crate::block_api
// Provides: {"impl_14"}
// Dependencies: {}
impl < Rate , const PAD : u8 , const ROUNDS : usize > ExtendableOutputCore for Sha3HasherCore < Rate , U0 , PAD , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , { type ReaderCore = Sha3ReaderCore < Rate , ROUNDS > ; # [inline] fn finalize_xof_core (& mut self , buffer : & mut Buffer < Self >) -> Self :: ReaderCore { let pos = buffer . get_pos () ; let mut block = buffer . pad_with_zeros () ; block [pos] = PAD ; let n = block . len () ; block [n - 1] |= 0x80 ; xor_block (& mut self . state , & block) ; keccak :: p1600 (& mut self . state , ROUNDS) ; Sha3ReaderCore :: new (& self . state) } }
};
}
