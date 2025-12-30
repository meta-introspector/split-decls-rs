// Generated macro for impl_1158 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_signimpl_1158 {
() => {
// Module: crate::crypto::aws_lc_rs::sign
// Provides: {"impl_1158"}
// Dependencies: {}
impl TryFrom < & PrivateKeyDer < '_ > > for EcdsaSigner { type Error = Error ; # [doc = " Parse `der` as any ECDSA key type, returning the first which works."] # [doc = ""] # [doc = " Both SEC1 (PEM section starting with 'BEGIN EC PRIVATE KEY') and PKCS8"] # [doc = " (PEM section starting with 'BEGIN PRIVATE KEY') encodings are supported."] fn try_from (der : & PrivateKeyDer < '_ >) -> Result < Self , Error > { if let Ok (ecdsa_p256) = Self :: new (der , SignatureScheme :: ECDSA_NISTP256_SHA256 , & signature :: ECDSA_P256_SHA256_ASN1_SIGNING ,) { return Ok (ecdsa_p256) ; } if let Ok (ecdsa_p384) = Self :: new (der , SignatureScheme :: ECDSA_NISTP384_SHA384 , & signature :: ECDSA_P384_SHA384_ASN1_SIGNING ,) { return Ok (ecdsa_p384) ; } if let Ok (ecdsa_p521) = Self :: new (der , SignatureScheme :: ECDSA_NISTP521_SHA512 , & signature :: ECDSA_P521_SHA512_ASN1_SIGNING ,) { return Ok (ecdsa_p521) ; } Err (Error :: General ("failed to parse ECDSA private key as PKCS#8 or SEC1" . into () ,)) } }
};
}
