// Generated macro for TLS_FUZZING_SUITE (const)
macro_rules! DepcrateTLS_FUZZING_SUITE {
() => {
// Module: crate
// Provides: {"TLS_FUZZING_SUITE"}
// Dependencies: {}
pub const TLS_FUZZING_SUITE : & Tls12CipherSuite = & Tls12CipherSuite { common : CipherSuiteCommon { suite : CipherSuite :: Unknown (0xff12) , hash_provider : & Hash , confidentiality_limit : u64 :: MAX , } , protocol_version : rustls :: version :: TLS12_VERSION , kx : KeyExchangeAlgorithm :: ECDHE , sign : & [SIGNATURE_SCHEME] , prf_provider : & tls12 :: PrfUsingHmac (& Hmac) , aead_alg : & Aead , } ;
};
}
