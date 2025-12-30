// Generated macro for impl_61 (impl)
macro_rules! Depcrate_kmacimpl_61 {
() => {
// Module: crate::kmac
// Provides: {"impl_61"}
// Dependencies: {}
impl Hasher for Kmac { fn update (& mut self , input : & [u8]) { self . state . update (input) } fn finalize (mut self , output : & mut [u8]) { self . state . update (right_encode (output . len () * 8) . value ()) ; self . state . finalize (output) } }
};
}
