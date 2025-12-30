// Generated macro for impl_27 (impl)
macro_rules! Depcrate_block_apiimpl_27 {
() => {
// Module: crate::block_api
// Provides: {"impl_27"}
// Dependencies: {}
impl < Rate , const ROUNDS : usize > fmt :: Debug for Sha3ReaderCore < Rate , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Sha3ReaderCore { ... }") } }
};
}
