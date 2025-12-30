// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl < T > PinnedServerCertVerifier < T > where T : AsRef < [rustls :: Certificate] > + Send + Sync , { pub fn new (certs : T) -> Self { Self { certs } } }
};
}
