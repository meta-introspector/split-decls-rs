// Generated macro for EcdsaSigner (struct)
macro_rules! Depcrate_crypto_aws_lc_rs_signEcdsaSigner {
() => {
// Module: crate::crypto::aws_lc_rs::sign
// Provides: {"EcdsaSigner"}
// Dependencies: {}
# [doc = " A [`SigningKey`] and [`Signer`] implementation for ECDSA."] # [doc = ""] # [doc = " Unlike [`RsaSigningKey`]/[`RsaSigner`], where we have one key that supports"] # [doc = " multiple signature schemes, we can use the same type for both traits here."] # [derive (Clone)] pub (super) struct EcdsaSigner { key : Arc < EcdsaKeyPair > , scheme : SignatureScheme , }
};
}
