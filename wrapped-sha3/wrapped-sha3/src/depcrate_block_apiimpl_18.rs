// Generated macro for impl_18 (impl)
macro_rules! Depcrate_block_apiimpl_18 {
() => {
// Module: crate::block_api
// Provides: {"impl_18"}
// Dependencies: {}
impl < Rate , OutputSize , const PAD : u8 , const ROUNDS : usize > fmt :: Debug for Sha3HasherCore < Rate , OutputSize , PAD , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , OutputSize : ArraySize + IsLessOrEqual < U200 , Output = True > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Sha3FixedCore<Rate, OutputSize, PAD, ROUNDS> { ... }") } }
};
}
