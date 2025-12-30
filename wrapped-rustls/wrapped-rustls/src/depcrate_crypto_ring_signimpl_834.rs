// Generated macro for impl_834 (impl)
macro_rules! Depcrate_crypto_ring_signimpl_834 {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"impl_834"}
// Dependencies: {}
impl TryFrom < & PrivatePkcs8KeyDer < '_ > > for Ed25519Signer { type Error = Error ; # [doc = " Parse `der` as an Ed25519 key."] # [doc = ""] # [doc = " Note that, at the time of writing, Ed25519 does not have wide support"] # [doc = " in browsers.  It is also not supported by the WebPKI, because the"] # [doc = " CA/Browser Forum Baseline Requirements do not support it for publicly"] # [doc = " trusted certificates."] fn try_from (der : & PrivatePkcs8KeyDer < '_ >) -> Result < Self , Self :: Error > { match Ed25519KeyPair :: from_pkcs8_maybe_unchecked (der . secret_pkcs8_der ()) { Ok (key_pair) => Ok (Self { key : Arc :: new (key_pair) , scheme : SignatureScheme :: ED25519 , }) , Err (e) => Err (Error :: General (format ! ("failed to parse Ed25519 private key: {e}"))) , } } }
};
}
