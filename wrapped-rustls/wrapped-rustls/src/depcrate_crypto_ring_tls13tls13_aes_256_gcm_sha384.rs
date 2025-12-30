// Generated macro for TLS13_AES_256_GCM_SHA384 (static)
macro_rules! Depcrate_crypto_ring_tls13TLS13_AES_256_GCM_SHA384 {
() => {
// Module: crate::crypto::ring::tls13
// Provides: {"TLS13_AES_256_GCM_SHA384"}
// Dependencies: {}
# [doc = " The TLS1.3 ciphersuite TLS_AES_256_GCM_SHA384"] pub static TLS13_AES_256_GCM_SHA384 : & Tls13CipherSuite = & Tls13CipherSuite { common : CipherSuiteCommon { suite : CipherSuite :: TLS13_AES_256_GCM_SHA384 , hash_provider : & super :: hash :: SHA384 , confidentiality_limit : 1 << 24 , } , protocol_version : TLS13_VERSION , hkdf_provider : & RingHkdf (hkdf :: HKDF_SHA384 , hmac :: HMAC_SHA384) , aead_alg : & Aes256GcmAead (AeadAlgorithm (& aead :: AES_256_GCM)) , quic : Some (& super :: quic :: KeyBuilder { packet_alg : & aead :: AES_256_GCM , header_alg : & aead :: quic :: AES_256 , confidentiality_limit : 1 << 23 , integrity_limit : 1 << 52 , }) , } ;
};
}
