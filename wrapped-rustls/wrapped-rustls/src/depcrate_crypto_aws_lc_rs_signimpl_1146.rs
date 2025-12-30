// Generated macro for impl_1146 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_signimpl_1146 {
() => {
// Module: crate::crypto::aws_lc_rs::sign
// Provides: {"impl_1146"}
// Dependencies: {}
impl RsaSigningKey { fn to_signer (& self , scheme : SignatureScheme) -> RsaSigner { let encoding : & dyn signature :: RsaEncoding = match scheme { SignatureScheme :: RSA_PKCS1_SHA256 => & signature :: RSA_PKCS1_SHA256 , SignatureScheme :: RSA_PKCS1_SHA384 => & signature :: RSA_PKCS1_SHA384 , SignatureScheme :: RSA_PKCS1_SHA512 => & signature :: RSA_PKCS1_SHA512 , SignatureScheme :: RSA_PSS_SHA256 => & signature :: RSA_PSS_SHA256 , SignatureScheme :: RSA_PSS_SHA384 => & signature :: RSA_PSS_SHA384 , SignatureScheme :: RSA_PSS_SHA512 => & signature :: RSA_PSS_SHA512 , _ => unreachable ! () , } ; RsaSigner { key : self . key . clone () , scheme , encoding , } } const SCHEMES : & [SignatureScheme] = & [SignatureScheme :: RSA_PSS_SHA512 , SignatureScheme :: RSA_PSS_SHA384 , SignatureScheme :: RSA_PSS_SHA256 , SignatureScheme :: RSA_PKCS1_SHA512 , SignatureScheme :: RSA_PKCS1_SHA384 , SignatureScheme :: RSA_PKCS1_SHA256 ,] ; }
};
}
