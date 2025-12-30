// Generated macro for impl_19 (impl)
macro_rules! Depcrate_block_apiimpl_19 {
() => {
// Module: crate::block_api
// Provides: {"impl_19"}
// Dependencies: {}
impl < Rate , OutputSize , const PAD : u8 , const ROUNDS : usize > Drop for Sha3HasherCore < Rate , OutputSize , PAD , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , OutputSize : ArraySize + IsLessOrEqual < U200 , Output = True > , { fn drop (& mut self) { # [cfg (feature = "zeroize")] { use digest :: zeroize :: Zeroize ; self . state . zeroize () ; } } }
};
}
