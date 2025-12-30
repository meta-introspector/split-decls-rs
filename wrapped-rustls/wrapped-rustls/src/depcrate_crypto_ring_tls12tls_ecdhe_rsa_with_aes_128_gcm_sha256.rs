// Generated macro for TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 (static)
macro_rules! Depcrate_crypto_ring_tls12TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 {
() => {
// Module: crate::crypto::ring::tls12
// Provides: {"TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256"}
// Dependencies: {}
# [doc = " The TLS1.2 ciphersuite TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256"] pub static TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 : & Tls12CipherSuite = & Tls12CipherSuite { common : CipherSuiteCommon { suite : CipherSuite :: TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 , hash_provider : & super :: hash :: SHA256 , confidentiality_limit : 1 << 24 , } , protocol_version : TLS12_VERSION , kx : KeyExchangeAlgorithm :: ECDHE , sign : TLS12_RSA_SCHEMES , aead_alg : & AES128_GCM , prf_provider : & PrfUsingHmac (& super :: hmac :: HMAC_SHA256) , } ;
};
}
