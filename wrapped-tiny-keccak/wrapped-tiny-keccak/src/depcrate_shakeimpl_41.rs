// Generated macro for impl_41 (impl)
macro_rules! Depcrate_shakeimpl_41 {
() => {
// Module: crate::shake
// Provides: {"impl_41"}
// Dependencies: {}
impl Hasher for Shake { fn update (& mut self , input : & [u8]) { self . state . update (input) ; } fn finalize (self , output : & mut [u8]) { self . state . finalize (output) ; } }
};
}
