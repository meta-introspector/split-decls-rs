// Generated macro for TLS13_FUZZING_SUITE (const)
macro_rules! DepcrateTLS13_FUZZING_SUITE {
() => {
// Module: crate
// Provides: {"TLS13_FUZZING_SUITE"}
// Dependencies: {}
pub const TLS13_FUZZING_SUITE : & Tls13CipherSuite = & Tls13CipherSuite { common : CipherSuiteCommon { suite : CipherSuite :: Unknown (0xff13) , hash_provider : & Hash , confidentiality_limit : u64 :: MAX , } , protocol_version : rustls :: version :: TLS13_VERSION , hkdf_provider : & tls13 :: HkdfUsingHmac (& Hmac) , aead_alg : & Aead , quic : None , } ;
};
}
