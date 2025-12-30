// Generated macro for impl_70 (impl)
macro_rules! Depcrate_tuple_hashimpl_70 {
() => {
// Module: crate::tuple_hash
// Provides: {"impl_70"}
// Dependencies: {}
impl Hasher for TupleHash { fn update (& mut self , input : & [u8]) { self . state . update (left_encode (input . len () * 8) . value ()) ; self . state . update (input) } fn finalize (mut self , output : & mut [u8]) { self . state . update (right_encode (output . len () * 8) . value ()) ; self . state . finalize (output) } }
};
}
