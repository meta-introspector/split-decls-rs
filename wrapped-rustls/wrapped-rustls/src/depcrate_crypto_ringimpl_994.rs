// Generated macro for impl_994 (impl)
macro_rules! Depcrate_crypto_ringimpl_994 {
() => {
// Module: crate::crypto::ring
// Provides: {"impl_994"}
// Dependencies: {}
impl KeyProvider for Ring { fn load_private_key (& self , key_der : PrivateKeyDer < 'static > ,) -> Result < Box < dyn SigningKey > , Error > { if let Ok (rsa) = RsaSigningKey :: try_from (& key_der) { return Ok (Box :: new (rsa)) ; } if let Ok (ecdsa) = EcdsaSigner :: try_from (& key_der) { return Ok (Box :: new (ecdsa)) ; } if let PrivateKeyDer :: Pkcs8 (pkcs8) = key_der { if let Ok (eddsa) = Ed25519Signer :: try_from (& pkcs8) { return Ok (Box :: new (eddsa)) ; } } Err (Error :: General ("failed to parse private key as RSA, ECDSA, or EdDSA" . into () ,)) } }
};
}
