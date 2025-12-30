// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < T > rustls :: ServerCertVerifier for PinnedServerCertVerifier < T > where T : AsRef < [rustls :: Certificate] > + Send + Sync , { fn verify_server_cert (& self , _roots : & rustls :: RootCertStore , presented_certs : & [rustls :: Certificate] , _dns_name : webpki :: DNSNameRef , _ocsp_response : & [u8] ,) -> Result < rustls :: ServerCertVerified , rustls :: TLSError > { let presented_cert = & presented_certs [0] ; for cert in self . certs . as_ref () { if presented_cert == cert { return Ok (rustls :: ServerCertVerified :: assertion ()) ; } } Err (rustls :: TLSError :: WebPKIError (webpki :: Error :: UnknownIssuer)) } }
};
}
