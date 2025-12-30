// Generated macro for Credentials (struct)
macro_rules! Depcrate_crypto_signerCredentials {
() => {
// Module: crate::crypto::signer
// Provides: {"Credentials"}
// Dependencies: {}
# [doc = " A packaged-together certificate chain, matching `SigningKey` and"] # [doc = " optional stapled OCSP response."] # [doc = ""] # [doc = " Note: this struct is also used to represent an [RFC 7250] raw public key,"] # [doc = " when the client/server is configured to use raw public keys instead of"] # [doc = " certificates."] # [doc = ""] # [doc = " [RFC 7250]: https://tools.ietf.org/html/rfc7250"] # [non_exhaustive] # [derive (Debug)] pub struct Credentials { # [doc = " The certificate chain or raw public key."] pub identity : Arc < Identity < 'static > > , # [doc = " The signing key matching the `identity`."] pub key : Box < dyn SigningKey > , # [doc = " An optional OCSP response from the certificate issuer,"] # [doc = " attesting to its continued validity."] pub ocsp : Option < Arc < [u8] > > , }
};
}
