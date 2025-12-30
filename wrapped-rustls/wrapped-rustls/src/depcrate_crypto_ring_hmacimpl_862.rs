// Generated macro for impl_862 (impl)
macro_rules! Depcrate_crypto_ring_hmacimpl_862 {
() => {
// Module: crate::crypto::ring::hmac
// Provides: {"impl_862"}
// Dependencies: {}
impl crypto :: hmac :: Key for Key { fn sign_concat (& self , first : & [u8] , middle : & [& [u8]] , last : & [u8]) -> crypto :: hmac :: Tag { let mut ctx = hmac :: Context :: with_key (& self . 0) ; ctx . update (first) ; for d in middle { ctx . update (d) ; } ctx . update (last) ; crypto :: hmac :: Tag :: new (ctx . sign () . as_ref ()) } fn tag_len (& self) -> usize { self . 0 . algorithm () . digest_algorithm () . output_len () } }
};
}
