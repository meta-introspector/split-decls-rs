// Generated macro for impl_1293 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_tls12impl_1293 {
() => {
// Module: crate::crypto::aws_lc_rs::tls12
// Provides: {"impl_1293"}
// Dependencies: {}
impl AsRef < [u8] > for Secret { fn as_ref (& self) -> & [u8] { match self { Self :: Master (ms) => ms . as_ref () , Self :: KeyExchange (kx) => kx . secret_bytes () , } } }
};
}
