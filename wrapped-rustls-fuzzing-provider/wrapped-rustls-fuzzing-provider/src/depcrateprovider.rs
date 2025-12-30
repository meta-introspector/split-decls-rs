// Generated macro for PROVIDER (const)
macro_rules! DepcratePROVIDER {
() => {
// Module: crate
// Provides: {"PROVIDER"}
// Dependencies: {}
# [doc = " This is a `CryptoProvider` that provides NO SECURITY and is for fuzzing only."] pub const PROVIDER : crypto :: CryptoProvider = crypto :: CryptoProvider { tls12_cipher_suites : Cow :: Borrowed (& [TLS_FUZZING_SUITE]) , tls13_cipher_suites : Cow :: Borrowed (& [TLS13_FUZZING_SUITE]) , kx_groups : Cow :: Borrowed (& [KEY_EXCHANGE_GROUP]) , signature_verification_algorithms : VERIFY_ALGORITHMS , secure_random : & Provider , key_provider : & Provider , ticketer_factory : & Provider , } ;
};
}
