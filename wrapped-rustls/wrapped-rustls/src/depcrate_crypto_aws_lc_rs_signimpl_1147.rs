// Generated macro for impl_1147 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_signimpl_1147 {
() => {
// Module: crate::crypto::aws_lc_rs::sign
// Provides: {"impl_1147"}
// Dependencies: {}
impl SigningKey for RsaSigningKey { fn choose_scheme (& self , offered : & [SignatureScheme]) -> Option < Box < dyn Signer > > { Self :: SCHEMES . iter () . find (| scheme | offered . contains (scheme)) . map (| & scheme | Box :: new (self . to_signer (scheme)) as Box < dyn Signer >) } fn public_key (& self) -> Option < SubjectPublicKeyInfoDer < '_ > > { Some (public_key_to_spki (& alg_id :: RSA_ENCRYPTION , self . key . public_key () ,)) } }
};
}
