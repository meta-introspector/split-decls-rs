// Generated macro for CertificateResult (struct)
macro_rules! DepcrateCertificateResult {
() => {
// Module: crate
// Provides: {"CertificateResult"}
// Dependencies: {}
# [doc = " Results from trying to load certificates from the platform's native store."] # [non_exhaustive] # [derive (Debug , Default)] pub struct CertificateResult { # [doc = " Any certificates that were successfully loaded."] pub certs : Vec < CertificateDer < 'static > > , # [doc = " Any errors encountered while loading certificates."] pub errors : Vec < Error > , }
};
}
