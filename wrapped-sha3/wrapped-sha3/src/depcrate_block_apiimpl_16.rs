// Generated macro for impl_16 (impl)
macro_rules! Depcrate_block_apiimpl_16 {
() => {
// Module: crate::block_api
// Provides: {"impl_16"}
// Dependencies: {}
impl < Rate , OutputSize , const PAD : u8 , const ROUNDS : usize > Reset for Sha3HasherCore < Rate , OutputSize , PAD , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , OutputSize : ArraySize + IsLessOrEqual < U200 , Output = True > , { # [inline] fn reset (& mut self) { * self = Default :: default () ; } }
};
}
