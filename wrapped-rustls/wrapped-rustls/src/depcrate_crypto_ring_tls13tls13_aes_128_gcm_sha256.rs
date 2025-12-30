// Generated macro for TLS13_AES_128_GCM_SHA256 (static)
macro_rules! Depcrate_crypto_ring_tls13TLS13_AES_128_GCM_SHA256 {
() => {
// Module: crate::crypto::ring::tls13
// Provides: {"TLS13_AES_128_GCM_SHA256"}
// Dependencies: {}
# [doc = " The TLS1.3 ciphersuite TLS_AES_128_GCM_SHA256"] pub static TLS13_AES_128_GCM_SHA256 : & Tls13CipherSuite = & Tls13CipherSuite { common : CipherSuiteCommon { suite : CipherSuite :: TLS13_AES_128_GCM_SHA256 , hash_provider : & super :: hash :: SHA256 , confidentiality_limit : 1 << 24 , } , protocol_version : TLS13_VERSION , hkdf_provider : & RingHkdf (hkdf :: HKDF_SHA256 , hmac :: HMAC_SHA256) , aead_alg : & Aes128GcmAead (AeadAlgorithm (& aead :: AES_128_GCM)) , quic : Some (& super :: quic :: KeyBuilder { packet_alg : & aead :: AES_128_GCM , header_alg : & aead :: quic :: AES_128 , confidentiality_limit : 1 << 23 , integrity_limit : 1 << 52 , }) , } ;
};
}
