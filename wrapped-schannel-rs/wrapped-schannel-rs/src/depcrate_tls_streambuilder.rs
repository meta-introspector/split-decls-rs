// Generated macro for Builder (struct)
macro_rules! Depcrate_tls_streamBuilder {
() => {
// Module: crate::tls_stream
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A builder type for `TlsStream`s."] pub struct Builder { domain : Option < Vec < u16 > > , use_sni : bool , accept_invalid_hostnames : bool , verify_callback : Option < Arc < dyn Fn (CertValidationResult) -> io :: Result < () > + Sync + Send > > , cert_store : Option < CertStore > , requested_application_protocols : Option < Vec < Vec < u8 > > > , }
};
}
