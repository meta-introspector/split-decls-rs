// Generated macro for impl_9 (impl)
macro_rules! Depcrate_block_apiimpl_9 {
() => {
// Module: crate::block_api
// Provides: {"impl_9"}
// Dependencies: {}
impl < Rate , OutputSize , const PAD : u8 , const ROUNDS : usize > BlockSizeUser for Sha3HasherCore < Rate , OutputSize , PAD , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , OutputSize : ArraySize + IsLessOrEqual < U200 , Output = True > , { type BlockSize = Rate ; }
};
}
