// Generated macro for impl_1109 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_pq_hybridimpl_1109 {
() => {
// Module: crate::crypto::aws_lc_rs::pq::hybrid
// Provides: {"impl_1109"}
// Dependencies: {}
impl HybridKeyExchange for ActiveHybrid { fn component (& self) -> (NamedGroup , & [u8]) { (self . classical . group () , self . classical . pub_key ()) } fn complete_component (self : Box < Self > , peer_pub_key : & [u8]) -> Result < SharedSecret , Error > { self . classical . complete (peer_pub_key) } fn into_key_exchange (self : Box < Self >) -> Box < dyn ActiveKeyExchange > { self } fn as_key_exchange (& self) -> & (dyn ActiveKeyExchange + 'static) { self } }
};
}
