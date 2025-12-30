// Generated macro for impl_1192 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_hmacimpl_1192 {
() => {
// Module: crate::crypto::aws_lc_rs::hmac
// Provides: {"impl_1192"}
// Dependencies: {}
impl crypto :: hmac :: Key for Key { fn sign_concat (& self , first : & [u8] , middle : & [& [u8]] , last : & [u8]) -> crypto :: hmac :: Tag { let mut ctx = hmac :: Context :: with_key (& self . 0) ; ctx . update (first) ; for d in middle { ctx . update (d) ; } ctx . update (last) ; crypto :: hmac :: Tag :: new (ctx . sign () . as_ref ()) } fn tag_len (& self) -> usize { self . 0 . algorithm () . digest_algorithm () . output_len () } }
};
}
