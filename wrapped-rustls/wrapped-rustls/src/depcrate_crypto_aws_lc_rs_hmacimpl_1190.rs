// Generated macro for impl_1190 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_hmacimpl_1190 {
() => {
// Module: crate::crypto::aws_lc_rs::hmac
// Provides: {"impl_1190"}
// Dependencies: {}
impl crypto :: hmac :: Hmac for Hmac { fn with_key (& self , key : & [u8]) -> Box < dyn crypto :: hmac :: Key > { Box :: new (Key (hmac :: Key :: new (* self . 0 , key))) } fn hash_output_len (& self) -> usize { self . 0 . digest_algorithm () . output_len () } fn fips (& self) -> bool { super :: fips () } }
};
}
