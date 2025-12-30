// Generated macro for TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 (static)
macro_rules! Depcrate_crypto_ring_tls12TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 {
() => {
// Module: crate::crypto::ring::tls12
// Provides: {"TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384"}
// Dependencies: {}
# [doc = " The TLS1.2 ciphersuite TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384"] pub static TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 : & Tls12CipherSuite = & Tls12CipherSuite { common : CipherSuiteCommon { suite : CipherSuite :: TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 , hash_provider : & super :: hash :: SHA384 , confidentiality_limit : 1 << 24 , } , protocol_version : TLS12_VERSION , kx : KeyExchangeAlgorithm :: ECDHE , sign : TLS12_ECDSA_SCHEMES , aead_alg : & AES256_GCM , prf_provider : & PrfUsingHmac (& super :: hmac :: HMAC_SHA384) , } ;
};
}
