// Generated macro for ServerCheckCertResolve (struct)
macro_rules! DepcrateServerCheckCertResolve {
() => {
// Module: crate
// Provides: {"ServerCheckCertResolve"}
// Dependencies: {}
# [derive (Default , Debug)] pub struct ServerCheckCertResolve { pub expected_sni : Option < DnsName < 'static > > , pub expected_sigalgs : Option < Vec < SignatureScheme > > , pub expected_alpn : Option < Vec < Vec < u8 > > > , pub expected_cipher_suites : Option < Vec < CipherSuite > > , pub expected_server_cert_types : Option < Vec < CertificateType > > , pub expected_client_cert_types : Option < Vec < CertificateType > > , pub expected_named_groups : Option < Vec < NamedGroup > > , }
};
}
