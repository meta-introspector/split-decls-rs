// Generated macro for impl_24 (impl)
macro_rules! Depcrate_block_apiimpl_24 {
() => {
// Module: crate::block_api
// Provides: {"impl_24"}
// Dependencies: {}
impl < Rate , const ROUNDS : usize > BlockSizeUser for Sha3ReaderCore < Rate , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , { type BlockSize = Rate ; }
};
}
