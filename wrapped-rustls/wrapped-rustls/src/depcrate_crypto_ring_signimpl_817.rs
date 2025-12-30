// Generated macro for impl_817 (impl)
macro_rules! Depcrate_crypto_ring_signimpl_817 {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"impl_817"}
// Dependencies: {}
impl SigningKey for RsaSigningKey { fn choose_scheme (& self , offered : & [SignatureScheme]) -> Option < Box < dyn Signer > > { Self :: SCHEMES . iter () . find (| scheme | offered . contains (scheme)) . map (| & scheme | Box :: new (self . to_signer (scheme)) as Box < dyn Signer >) } fn public_key (& self) -> Option < SubjectPublicKeyInfoDer < '_ > > { Some (public_key_to_spki (& alg_id :: RSA_ENCRYPTION , self . key . public_key () ,)) } }
};
}
