// Generated macro for impl_1223 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_quicimpl_1223 {
() => {
// Module: crate::crypto::aws_lc_rs::quic
// Provides: {"impl_1223"}
// Dependencies: {}
impl PacketKey { pub (crate) fn new (key : AeadKey , iv : Iv , confidentiality_limit : u64 , integrity_limit : u64 , aead_algorithm : & 'static aead :: Algorithm ,) -> Self { Self { key : aead :: LessSafeKey :: new (aead :: UnboundKey :: new (aead_algorithm , key . as_ref ()) . unwrap () ,) , iv , confidentiality_limit , integrity_limit , } } }
};
}
