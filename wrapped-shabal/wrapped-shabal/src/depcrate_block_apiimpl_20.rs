// Generated macro for impl_20 (impl)
macro_rules! Depcrate_block_apiimpl_20 {
() => {
// Module: crate::block_api
// Provides: {"impl_20"}
// Dependencies: {}
impl Drop for ShabalVarCore { fn drop (& mut self) { # [cfg (feature = "zeroize")] { use digest :: zeroize :: Zeroize ; self . a . zeroize () ; self . b . zeroize () ; self . c . zeroize () ; self . w . zeroize () ; } } }
};
}
