// Generated macro for provider (function)
macro_rules! Depcrateprovider {
() => {
// Module: crate
// Provides: {"provider"}
// Dependencies: {}
pub fn provider () -> CryptoProvider { # [cfg_attr (not (feature = "aws-lc-rs-unstable") , allow (unused_mut))] let mut provider = rustls :: crypto :: aws_lc_rs :: default_provider () ; # [cfg (feature = "aws-lc-rs-unstable")] { provider . signature_verification_algorithms = SUPPORTED_SIG_ALGS ; provider . key_provider = & key_provider :: PqAwsLcRs ; } provider }
};
}
