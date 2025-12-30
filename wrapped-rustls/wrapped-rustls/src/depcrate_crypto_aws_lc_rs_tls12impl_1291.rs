// Generated macro for impl_1291 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_tls12impl_1291 {
() => {
// Module: crate::crypto::aws_lc_rs::tls12
// Provides: {"impl_1291"}
// Dependencies: {}
impl PrfSecret for Tls12PrfSecret { fn prf (& self , output : & mut [u8] , label : & [u8] , seed : & [u8]) { let derived = tls_prf :: Secret :: new (self . alg , self . secret . as_ref ()) . unwrap () . derive (label , seed , output . len ()) . unwrap () ; output . copy_from_slice (derived . as_ref ()) ; } }
};
}
