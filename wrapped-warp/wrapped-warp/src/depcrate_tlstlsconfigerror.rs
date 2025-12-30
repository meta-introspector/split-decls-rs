// Generated macro for TlsConfigError (enum)
macro_rules! Depcrate_tlsTlsConfigError {
() => {
// Module: crate::tls
// Provides: {"TlsConfigError"}
// Dependencies: {}
# [doc = " Represents errors that can occur building the TlsConfig"] # [derive (Debug)] pub (crate) enum TlsConfigError { Io (io :: Error) , # [doc = " An Error parsing the Certificate"] CertParseError , # [doc = " Identity PEM is invalid"] InvalidIdentityPem , # [doc = " Identity PEM is missing a private key such as RSA, ECC or PKCS8"] MissingPrivateKey , # [doc = " Unknown private key format"] UnknownPrivateKeyFormat , # [doc = " An error from an empty key"] EmptyKey , # [doc = " An error from an invalid key"] InvalidKey (TlsError) , }
};
}
