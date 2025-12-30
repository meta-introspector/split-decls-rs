// Generated macro for impl_11 (impl)
macro_rules! Depcrate_block_apiimpl_11 {
() => {
// Module: crate::block_api
// Provides: {"impl_11"}
// Dependencies: {}
impl < Rate , OutputSize , const PAD : u8 , const ROUNDS : usize > OutputSizeUser for Sha3HasherCore < Rate , OutputSize , PAD , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , OutputSize : ArraySize + IsLessOrEqual < U200 , Output = True > , { type OutputSize = OutputSize ; }
};
}
