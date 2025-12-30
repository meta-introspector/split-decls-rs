// Generated macro for impl_818 (impl)
macro_rules! Depcrate_crypto_ring_signimpl_818 {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"impl_818"}
// Dependencies: {}
impl TryFrom < & PrivateKeyDer < '_ > > for RsaSigningKey { type Error = Error ; # [doc = " Make a new `RsaSigningKey` from a DER encoding, in either"] # [doc = " PKCS#1 or PKCS#8 format."] fn try_from (der : & PrivateKeyDer < '_ >) -> Result < Self , Self :: Error > { let key_pair = match der { PrivateKeyDer :: Pkcs1 (pkcs1) => RsaKeyPair :: from_der (pkcs1 . secret_pkcs1_der ()) , PrivateKeyDer :: Pkcs8 (pkcs8) => RsaKeyPair :: from_pkcs8 (pkcs8 . secret_pkcs8_der ()) , _ => { return Err (Error :: General ("failed to parse RSA private key as either PKCS#1 or PKCS#8" . into () ,)) ; } } . map_err (| key_rejected | { Error :: General (format ! ("failed to parse RSA private key: {key_rejected}")) }) ? ; Ok (Self { key : Arc :: new (key_pair) , }) } }
};
}
