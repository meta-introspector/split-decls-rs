// Generated macro for ConfigVerifierExt (trait)
macro_rules! DepcrateConfigVerifierExt {
() => {
// Module: crate
// Provides: {"ConfigVerifierExt"}
// Dependencies: {}
# [doc = " Extension trait to help build a [`ClientConfig`] with the platform verifier."] pub trait ConfigVerifierExt { # [doc = " Build a [`ClientConfig`] with the platform verifier and the default `CryptoProvider`."] # [doc = ""] # [doc = " ```rust"] # [doc = " use rustls::ClientConfig;"] # [doc = " use rustls_platform_verifier::ConfigVerifierExt;"] # [doc = " let config = ClientConfig::with_platform_verifier();"] # [doc = " ```"] fn with_platform_verifier () -> Result < ClientConfig , rustls :: Error > ; }
};
}
