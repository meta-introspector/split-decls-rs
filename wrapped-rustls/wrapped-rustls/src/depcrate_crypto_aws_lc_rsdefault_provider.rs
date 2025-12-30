// Generated macro for DEFAULT_PROVIDER (const)
macro_rules! Depcrate_crypto_aws_lc_rsDEFAULT_PROVIDER {
() => {
// Module: crate::crypto::aws_lc_rs
// Provides: {"DEFAULT_PROVIDER"}
// Dependencies: {}
# [doc = " The default `CryptoProvider` backed by aws-lc-rs."] pub const DEFAULT_PROVIDER : CryptoProvider = CryptoProvider { tls12_cipher_suites : Cow :: Borrowed (DEFAULT_TLS12_CIPHER_SUITES) , tls13_cipher_suites : Cow :: Borrowed (DEFAULT_TLS13_CIPHER_SUITES) , kx_groups : Cow :: Borrowed (DEFAULT_KX_GROUPS) , signature_verification_algorithms : SUPPORTED_SIG_ALGS , secure_random : & AwsLcRs , key_provider : & AwsLcRs , ticketer_factory : & AwsLcRs , } ;
};
}
