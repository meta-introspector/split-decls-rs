// Generated macro for impl_48 (impl)
macro_rules! Depcrate_sha3impl_48 {
() => {
// Module: crate::sha3
// Provides: {"impl_48"}
// Dependencies: {}
impl Hasher for Sha3 { fn update (& mut self , input : & [u8]) { self . state . update (input) ; } fn finalize (self , output : & mut [u8]) { self . state . finalize (output) ; } }
};
}
