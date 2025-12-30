// Generated macro for impl_189 (impl)
macro_rules! Depcrateimpl_189 {
() => {
// Module: crate
// Provides: {"impl_189"}
// Dependencies: {}
impl ConfigVerifierExt for ClientConfig { fn with_platform_verifier () -> Result < ClientConfig , rustls :: Error > { Ok (ClientConfig :: builder () . with_platform_verifier () ? . with_no_client_auth ()) } }
};
}
