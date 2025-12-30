// Generated macro for impl_1082 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeimpl_1082 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"impl_1082"}
// Dependencies: {}
impl LabeledSuiteId { # [doc = " The suite ID encoding depends on the context of use. In the general HPKE context,"] # [doc = " we use a \"HPKE\" prefix and encode the entire ciphersuite. In the KEM context we use a"] # [doc = " \"KEM\" prefix and only encode the KEM ID."] # [doc = ""] # [doc = " See the bottom of [RFC 9180 §4](https://www.rfc-editor.org/rfc/rfc9180.html#section-4)"] # [doc = " for more information."] fn encoded (& self) -> Vec < u8 > { match self { Self :: Hpke (suite) => [& b"HPKE" [..] , & u16 :: from (suite . kem) . to_be_bytes () , & u16 :: from (suite . sym . kdf_id) . to_be_bytes () , & u16 :: from (suite . sym . aead_id) . to_be_bytes () ,] . concat () , Self :: Kem (kem) => [& b"KEM" [..] , & u16 :: from (* kem) . to_be_bytes ()] . concat () , } } }
};
}
