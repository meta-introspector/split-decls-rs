// Generated macro for impl_1180 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_hashimpl_1180 {
() => {
// Module: crate::crypto::aws_lc_rs::hash
// Provides: {"impl_1180"}
// Dependencies: {}
impl crypto :: hash :: Context for Context { fn fork_finish (& self) -> crypto :: hash :: Output { convert (self . 0 . clone () . finish ()) } fn fork (& self) -> Box < dyn crypto :: hash :: Context > { Box :: new (Self (self . 0 . clone ())) } fn finish (self : Box < Self >) -> crypto :: hash :: Output { convert (self . 0 . finish ()) } fn update (& mut self , data : & [u8]) { self . 0 . update (data) ; } }
};
}
