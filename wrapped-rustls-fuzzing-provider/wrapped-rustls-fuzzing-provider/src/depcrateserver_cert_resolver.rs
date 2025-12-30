// Generated macro for server_cert_resolver (function)
macro_rules! Depcrateserver_cert_resolver {
() => {
// Module: crate
// Provides: {"server_cert_resolver"}
// Dependencies: {}
pub fn server_cert_resolver () -> Arc < dyn ServerCredentialResolver > { let cert = CertificateDer :: from (& include_bytes ! ("../../test-ca/ecdsa-p256/end.der") [..]) ; let credentials = Credentials :: new_unchecked (Arc :: new (Identity :: from_cert_chain (vec ! [cert]) . unwrap ()) , Box :: new (SigningKey) ,) ; Arc :: new (DummyCert (credentials . into ())) }
};
}
