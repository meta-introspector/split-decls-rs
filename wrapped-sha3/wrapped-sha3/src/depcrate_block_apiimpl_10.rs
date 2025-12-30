// Generated macro for impl_10 (impl)
macro_rules! Depcrate_block_apiimpl_10 {
() => {
// Module: crate::block_api
// Provides: {"impl_10"}
// Dependencies: {}
impl < Rate , OutputSize , const PAD : u8 , const ROUNDS : usize > BufferKindUser for Sha3HasherCore < Rate , OutputSize , PAD , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , OutputSize : ArraySize + IsLessOrEqual < U200 , Output = True > , { type BufferKind = Eager ; }
};
}
