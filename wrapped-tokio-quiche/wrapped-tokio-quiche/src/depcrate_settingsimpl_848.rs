// Generated macro for impl_848 (impl)
macro_rules! Depcrate_settingsimpl_848 {
() => {
// Module: crate::settings
// Provides: {"impl_848"}
// Dependencies: {}
impl < 'a > ConnectionParams < 'a > { # [doc = " Creates [`ConnectionParams`] for a QUIC server."] # [doc = " Servers should always specify TLS credentials."] # [inline] pub fn new_server (settings : QuicSettings , tls_cert : TlsCertificatePaths < 'a > , hooks : Hooks ,) -> Self { Self { settings , tls_cert : Some (tls_cert) , hooks , session : None , } } # [doc = " Creates [`ConnectionParams`] for a QUIC client."] # [doc = " Clients may enable mTLS by specifying TLS credentials."] # [inline] pub fn new_client (settings : QuicSettings , tls_cert : Option < TlsCertificatePaths < 'a > > , hooks : Hooks ,) -> Self { Self { settings , tls_cert , hooks , session : None , } } }
};
}
