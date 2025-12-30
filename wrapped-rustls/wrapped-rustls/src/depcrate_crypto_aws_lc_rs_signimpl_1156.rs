// Generated macro for impl_1156 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_signimpl_1156 {
() => {
// Module: crate::crypto::aws_lc_rs::sign
// Provides: {"impl_1156"}
// Dependencies: {}
impl SigningKey for EcdsaSigner { fn choose_scheme (& self , offered : & [SignatureScheme]) -> Option < Box < dyn Signer > > { if offered . contains (& self . scheme) { Some (Box :: new (self . clone ())) } else { None } } fn public_key (& self) -> Option < SubjectPublicKeyInfoDer < '_ > > { let id = match self . scheme { SignatureScheme :: ECDSA_NISTP256_SHA256 => alg_id :: ECDSA_P256 , SignatureScheme :: ECDSA_NISTP384_SHA384 => alg_id :: ECDSA_P384 , SignatureScheme :: ECDSA_NISTP521_SHA512 => alg_id :: ECDSA_P521 , _ => unreachable ! () , } ; Some (public_key_to_spki (& id , self . key . public_key ())) } }
};
}
