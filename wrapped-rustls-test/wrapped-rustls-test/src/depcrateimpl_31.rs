// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl ServerConfigExt for rustls :: ConfigBuilder < ServerConfig , rustls :: WantsVerifier > { fn finish (self , kt : KeyType) -> ServerConfig { self . with_no_client_auth () . with_single_cert (kt . identity () , kt . key ()) . unwrap () } }
};
}
