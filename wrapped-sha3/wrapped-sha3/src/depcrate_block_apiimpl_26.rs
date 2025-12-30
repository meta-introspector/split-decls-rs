// Generated macro for impl_26 (impl)
macro_rules! Depcrate_block_apiimpl_26 {
() => {
// Module: crate::block_api
// Provides: {"impl_26"}
// Dependencies: {}
impl < Rate , const ROUNDS : usize > Drop for Sha3ReaderCore < Rate , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , { fn drop (& mut self) { # [cfg (feature = "zeroize")] { use digest :: zeroize :: Zeroize ; self . state . zeroize () ; } } }
};
}
