// Generated macro for impl_30 (impl)
macro_rules! Depcrate_block_apiimpl_30 {
() => {
// Module: crate::block_api
// Provides: {"impl_30"}
// Dependencies: {}
impl Drop for Sha512VarCore { fn drop (& mut self) { # [cfg (feature = "zeroize")] { use digest :: zeroize :: Zeroize ; self . state . zeroize () ; self . block_len . zeroize () ; } } }
};
}
