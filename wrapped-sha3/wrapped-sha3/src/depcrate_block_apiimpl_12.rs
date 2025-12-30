// Generated macro for impl_12 (impl)
macro_rules! Depcrate_block_apiimpl_12 {
() => {
// Module: crate::block_api
// Provides: {"impl_12"}
// Dependencies: {}
impl < Rate , OutputSize , const PAD : u8 , const ROUNDS : usize > UpdateCore for Sha3HasherCore < Rate , OutputSize , PAD , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , OutputSize : ArraySize + IsLessOrEqual < U200 , Output = True > , { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { for block in blocks { xor_block (& mut self . state , block) ; keccak :: p1600 (& mut self . state , ROUNDS) ; } } }
};
}
