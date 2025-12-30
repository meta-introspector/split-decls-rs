// Generated macro for PrivateKey (enum)
macro_rules! Depcrate_cert_contextPrivateKey {
() => {
// Module: crate::cert_context
// Provides: {"PrivateKey"}
// Dependencies: {}
# [doc = " The private key associated with a certificate context."] pub enum PrivateKey { # [doc = " A CryptoAPI provider."] CryptProv (CryptProv) , # [doc = " A CNG provider."] NcryptKey (NcryptKey) , }
};
}
