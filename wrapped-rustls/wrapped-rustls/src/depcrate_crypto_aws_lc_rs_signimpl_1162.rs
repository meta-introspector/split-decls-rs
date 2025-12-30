// Generated macro for impl_1162 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_signimpl_1162 {
() => {
// Module: crate::crypto::aws_lc_rs::sign
// Provides: {"impl_1162"}
// Dependencies: {}
impl SigningKey for Ed25519Signer { fn choose_scheme (& self , offered : & [SignatureScheme]) -> Option < Box < dyn Signer > > { if offered . contains (& self . scheme) { Some (Box :: new (self . clone ())) } else { None } } fn public_key (& self) -> Option < SubjectPublicKeyInfoDer < '_ > > { Some (public_key_to_spki (& alg_id :: ED25519 , self . key . public_key ())) } }
};
}
