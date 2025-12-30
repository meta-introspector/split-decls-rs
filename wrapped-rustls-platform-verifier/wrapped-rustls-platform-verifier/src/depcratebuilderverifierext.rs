// Generated macro for BuilderVerifierExt (trait)
macro_rules! DepcrateBuilderVerifierExt {
() => {
// Module: crate
// Provides: {"BuilderVerifierExt"}
// Dependencies: {}
# [doc = " Extension trait to help configure [`ClientConfig`]s with the platform verifier."] pub trait BuilderVerifierExt { # [doc = " Configures the `ClientConfig` with the platform verifier."] # [doc = ""] # [doc = " ```rust"] # [doc = " use rustls::ClientConfig;"] # [doc = " use rustls_platform_verifier::BuilderVerifierExt;"] # [doc = " let config = ClientConfig::builder()"] # [doc = "     .with_platform_verifier()"] # [doc = "     .unwrap()"] # [doc = "     .with_no_client_auth();"] # [doc = " ```"] fn with_platform_verifier (self ,) -> Result < ConfigBuilder < ClientConfig , WantsClientCert > , rustls :: Error > ; }
};
}
