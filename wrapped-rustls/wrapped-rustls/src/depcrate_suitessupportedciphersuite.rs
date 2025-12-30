// Generated macro for SupportedCipherSuite (enum)
macro_rules! Depcrate_suitesSupportedCipherSuite {
() => {
// Module: crate::suites
// Provides: {"SupportedCipherSuite"}
// Dependencies: {}
# [doc = " A cipher suite supported by rustls."] # [doc = ""] # [doc = " This type carries both configuration and implementation. Compare with"] # [doc = " [`CipherSuite`], which carries solely a cipher suite identifier."] # [non_exhaustive] # [derive (Clone , Copy , PartialEq)] pub enum SupportedCipherSuite { # [doc = " A TLS 1.2 cipher suite"] Tls12 (& 'static Tls12CipherSuite) , # [doc = " A TLS 1.3 cipher suite"] Tls13 (& 'static Tls13CipherSuite) , }
};
}
