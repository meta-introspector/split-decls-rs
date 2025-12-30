// Generated macro for impl_642 (impl)
macro_rules! Depcrate_secure_transportimpl_642 {
() => {
// Module: crate::secure_transport
// Provides: {"impl_642"}
// Dependencies: {}
impl SslClientCertificateState { # [doc = " A client certificate has not been requested or sent."] pub const NONE : Self = Self (kSSLClientCertNone) ; # [doc = " A client certificate has been received but has failed to validate."] pub const REJECTED : Self = Self (kSSLClientCertRejected) ; # [doc = " A client certificate has been requested but not recieved."] pub const REQUESTED : Self = Self (kSSLClientCertRequested) ; # [doc = " A client certificate has been received and successfully validated."] pub const SENT : Self = Self (kSSLClientCertSent) ; }
};
}
