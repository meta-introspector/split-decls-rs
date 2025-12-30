// Generated macro for impl_17 (impl)
macro_rules! Depcrate_block_apiimpl_17 {
() => {
// Module: crate::block_api
// Provides: {"impl_17"}
// Dependencies: {}
impl < Rate , OutputSize , const PAD : u8 , const ROUNDS : usize > AlgorithmName for Sha3HasherCore < Rate , OutputSize , PAD , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , OutputSize : ArraySize + IsLessOrEqual < U200 , Output = True > , { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Sha3") } }
};
}
