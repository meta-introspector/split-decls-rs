// Generated macro for impl_867 (impl)
macro_rules! Depcrate_tlsimpl_867 {
() => {
// Module: crate::tls
// Provides: {"impl_867"}
// Dependencies: {}
impl fmt :: Display for TlsConfigError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TlsConfigError :: Io (err) => err . fmt (f) , TlsConfigError :: CertParseError => write ! (f , "certificate parse error") , TlsConfigError :: UnknownPrivateKeyFormat => write ! (f , "unknown private key format") , TlsConfigError :: MissingPrivateKey => write ! (f , "Identity PEM is missing a private key such as RSA, ECC or PKCS8") , TlsConfigError :: InvalidIdentityPem => write ! (f , "identity PEM is invalid") , TlsConfigError :: EmptyKey => write ! (f , "key contains no private key") , TlsConfigError :: InvalidKey (err) => write ! (f , "key contains an invalid key, {}" , err) , } } }
};
}
