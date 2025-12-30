// Generated macro for SingleCredential (struct)
macro_rules! Depcrate_crypto_signerSingleCredential {
() => {
// Module: crate::crypto::signer
// Provides: {"SingleCredential"}
// Dependencies: {}
# [doc = " Server certificate resolver which always resolves to the same identity and key."] # [doc = ""] # [doc = " For use with [`ConfigBuilder::with_server_credential_resolver()`] or"] # [doc = " [`ConfigBuilder::with_client_credential_resolver()`]."] # [doc = ""] # [doc = " [`ConfigBuilder::with_server_credential_resolver()`]: crate::ConfigBuilder::with_server_credential_resolver"] # [doc = " [`ConfigBuilder::with_client_credential_resolver()`]: crate::ConfigBuilder::with_client_credential_resolver"] # [derive (Debug)] pub struct SingleCredential { credentials : Credentials , types : & 'static [CertificateType] , }
};
}
