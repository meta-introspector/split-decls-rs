// Generated macro for TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256 (static)
macro_rules! Depcrate_crypto_aws_lc_rs_tls12TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256 {
() => {
// Module: crate::crypto::aws_lc_rs::tls12
// Provides: {"TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256"}
// Dependencies: {}
# [doc = " The TLS1.2 ciphersuite TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256."] pub static TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256 : & Tls12CipherSuite = & Tls12CipherSuite { common : CipherSuiteCommon { suite : CipherSuite :: TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256 , hash_provider : & super :: hash :: SHA256 , confidentiality_limit : u64 :: MAX , } , protocol_version : TLS12_VERSION , kx : KeyExchangeAlgorithm :: ECDHE , sign : TLS12_ECDSA_SCHEMES , aead_alg : & ChaCha20Poly1305 , prf_provider : & Tls12Prf (& tls_prf :: P_SHA256) , } ;
};
}
