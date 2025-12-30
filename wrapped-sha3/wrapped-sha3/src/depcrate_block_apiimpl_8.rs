// Generated macro for impl_8 (impl)
macro_rules! Depcrate_block_apiimpl_8 {
() => {
// Module: crate::block_api
// Provides: {"impl_8"}
// Dependencies: {}
impl < Rate , OutputSize , const PAD : u8 , const ROUNDS : usize > HashMarker for Sha3HasherCore < Rate , OutputSize , PAD , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , OutputSize : ArraySize + IsLessOrEqual < U200 , Output = True > , { }
};
}
