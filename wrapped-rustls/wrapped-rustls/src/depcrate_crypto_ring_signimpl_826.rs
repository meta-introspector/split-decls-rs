// Generated macro for impl_826 (impl)
macro_rules! Depcrate_crypto_ring_signimpl_826 {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"impl_826"}
// Dependencies: {}
impl SigningKey for EcdsaSigner { fn choose_scheme (& self , offered : & [SignatureScheme]) -> Option < Box < dyn Signer > > { if offered . contains (& self . scheme) { Some (Box :: new (self . clone ())) } else { None } } fn public_key (& self) -> Option < SubjectPublicKeyInfoDer < '_ > > { let id = match self . scheme { SignatureScheme :: ECDSA_NISTP256_SHA256 => alg_id :: ECDSA_P256 , SignatureScheme :: ECDSA_NISTP384_SHA384 => alg_id :: ECDSA_P384 , _ => unreachable ! () , } ; Some (public_key_to_spki (& id , self . key . public_key ())) } }
};
}
