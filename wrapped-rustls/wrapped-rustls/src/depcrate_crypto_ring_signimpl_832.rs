// Generated macro for impl_832 (impl)
macro_rules! Depcrate_crypto_ring_signimpl_832 {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"impl_832"}
// Dependencies: {}
impl SigningKey for Ed25519Signer { fn choose_scheme (& self , offered : & [SignatureScheme]) -> Option < Box < dyn Signer > > { if offered . contains (& self . scheme) { Some (Box :: new (self . clone ())) } else { None } } fn public_key (& self) -> Option < SubjectPublicKeyInfoDer < '_ > > { Some (public_key_to_spki (& alg_id :: ED25519 , self . key . public_key ())) } }
};
}
