// Generated macro for impl_54 (impl)
macro_rules! Depcrate_cshakeimpl_54 {
() => {
// Module: crate::cshake
// Provides: {"impl_54"}
// Dependencies: {}
impl Hasher for CShake { fn update (& mut self , input : & [u8]) { self . state . update (input) ; } fn finalize (self , output : & mut [u8]) { self . state . finalize (output) ; } }
};
}
