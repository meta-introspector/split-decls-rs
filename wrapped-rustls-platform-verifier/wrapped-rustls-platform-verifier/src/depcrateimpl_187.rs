// Generated macro for impl_187 (impl)
macro_rules! Depcrateimpl_187 {
() => {
// Module: crate
// Provides: {"impl_187"}
// Dependencies: {}
impl BuilderVerifierExt for ConfigBuilder < ClientConfig , WantsVerifier > { fn with_platform_verifier (self ,) -> Result < ConfigBuilder < ClientConfig , WantsClientCert > , rustls :: Error > { let verifier = Verifier :: new (self . crypto_provider () . clone ()) ? ; Ok (self . dangerous () . with_custom_certificate_verifier (Arc :: new (verifier))) } }
};
}
