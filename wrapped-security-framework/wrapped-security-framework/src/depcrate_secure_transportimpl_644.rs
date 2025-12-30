// Generated macro for impl_644 (impl)
macro_rules! Depcrate_secure_transportimpl_644 {
() => {
// Module: crate::secure_transport
// Provides: {"impl_644"}
// Dependencies: {}
impl SslProtocol { # [doc = " All supported TLS/SSL versions are accepted."] pub const ALL : Self = Self (kSSLProtocolAll) ; # [doc = " The `DTLSv1` protocol is preferred."] pub const DTLS1 : Self = Self (kDTLSProtocol1) ; # [doc = " Only the SSL 2.0 protocol is accepted."] pub const SSL2 : Self = Self (kSSLProtocol2) ; # [doc = " The SSL 3.0 protocol is preferred, though SSL 2.0 may be used if the peer does not support"] # [doc = " SSL 3.0."] pub const SSL3 : Self = Self (kSSLProtocol3) ; # [doc = " Only the SSL 3.0 protocol is accepted."] pub const SSL3_ONLY : Self = Self (kSSLProtocol3Only) ; # [doc = " The TLS 1.0 protocol is preferred, though lower versions may be used"] # [doc = " if the peer does not support TLS 1.0."] pub const TLS1 : Self = Self (kTLSProtocol1) ; # [doc = " The TLS 1.1 protocol is preferred, though lower versions may be used"] # [doc = " if the peer does not support TLS 1.1."] pub const TLS11 : Self = Self (kTLSProtocol11) ; # [doc = " The TLS 1.2 protocol is preferred, though lower versions may be used"] # [doc = " if the peer does not support TLS 1.2."] pub const TLS12 : Self = Self (kTLSProtocol12) ; # [doc = " The TLS 1.3 protocol is preferred, though lower versions may be used"] # [doc = " if the peer does not support TLS 1.3."] pub const TLS13 : Self = Self (kTLSProtocol13) ; # [doc = " Only the TLS 1.0 protocol is accepted."] pub const TLS1_ONLY : Self = Self (kTLSProtocol1Only) ; # [doc = " No protocol has been or should be negotiated or specified; use the default."] pub const UNKNOWN : Self = Self (kSSLProtocolUnknown) ; }
};
}
