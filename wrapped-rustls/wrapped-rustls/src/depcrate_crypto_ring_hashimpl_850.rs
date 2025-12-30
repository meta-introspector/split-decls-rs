// Generated macro for impl_850 (impl)
macro_rules! Depcrate_crypto_ring_hashimpl_850 {
() => {
// Module: crate::crypto::ring::hash
// Provides: {"impl_850"}
// Dependencies: {}
impl crypto :: hash :: Context for Context { fn fork_finish (& self) -> crypto :: hash :: Output { convert (self . 0 . clone () . finish ()) } fn fork (& self) -> Box < dyn crypto :: hash :: Context > { Box :: new (Self (self . 0 . clone ())) } fn finish (self : Box < Self >) -> crypto :: hash :: Output { convert (self . 0 . finish ()) } fn update (& mut self , data : & [u8]) { self . 0 . update (data) ; } }
};
}
