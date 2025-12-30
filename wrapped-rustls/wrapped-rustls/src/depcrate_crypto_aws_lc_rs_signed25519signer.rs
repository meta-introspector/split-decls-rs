// Generated macro for Ed25519Signer (struct)
macro_rules! Depcrate_crypto_aws_lc_rs_signEd25519Signer {
() => {
// Module: crate::crypto::aws_lc_rs::sign
// Provides: {"Ed25519Signer"}
// Dependencies: {}
# [doc = " A [`SigningKey`] and [`Signer`] implementation for ED25519."] # [doc = ""] # [doc = " Unlike [`RsaSigningKey`]/[`RsaSigner`], where we have one key that supports"] # [doc = " multiple signature schemes, we can use the same type for both traits here."] # [derive (Clone)] pub (super) struct Ed25519Signer { key : Arc < Ed25519KeyPair > , scheme : SignatureScheme , }
};
}
