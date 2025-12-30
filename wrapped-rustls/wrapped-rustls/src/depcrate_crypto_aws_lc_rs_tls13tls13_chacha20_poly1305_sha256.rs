// Generated macro for TLS13_CHACHA20_POLY1305_SHA256 (static)
macro_rules! Depcrate_crypto_aws_lc_rs_tls13TLS13_CHACHA20_POLY1305_SHA256 {
() => {
// Module: crate::crypto::aws_lc_rs::tls13
// Provides: {"TLS13_CHACHA20_POLY1305_SHA256"}
// Dependencies: {}
# [doc = " The TLS1.3 ciphersuite TLS_CHACHA20_POLY1305_SHA256"] pub static TLS13_CHACHA20_POLY1305_SHA256 : & Tls13CipherSuite = & Tls13CipherSuite { common : CipherSuiteCommon { suite : CipherSuite :: TLS13_CHACHA20_POLY1305_SHA256 , hash_provider : & super :: hash :: SHA256 , confidentiality_limit : u64 :: MAX , } , protocol_version : TLS13_VERSION , hkdf_provider : & AwsLcHkdf (hkdf :: HKDF_SHA256 , hmac :: HMAC_SHA256) , aead_alg : & Chacha20Poly1305Aead (AeadAlgorithm (& aead :: CHACHA20_POLY1305)) , quic : Some (& super :: quic :: KeyBuilder { packet_alg : & aead :: CHACHA20_POLY1305 , header_alg : & aead :: quic :: CHACHA20 , confidentiality_limit : u64 :: MAX , integrity_limit : 1 << 36 , }) , } ;
};
}
