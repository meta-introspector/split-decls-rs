// Generated macro for impl_828 (impl)
macro_rules! Depcrate_crypto_ring_signimpl_828 {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"impl_828"}
// Dependencies: {}
impl TryFrom < & PrivateKeyDer < '_ > > for EcdsaSigner { type Error = Error ; # [doc = " Parse `der` as any ECDSA key type, returning the first which works."] # [doc = ""] # [doc = " Both SEC1 (PEM section starting with 'BEGIN EC PRIVATE KEY') and PKCS8"] # [doc = " (PEM section starting with 'BEGIN PRIVATE KEY') encodings are supported."] fn try_from (der : & PrivateKeyDer < '_ >) -> Result < Self , Self :: Error > { if let Ok (ecdsa_p256) = Self :: new (der , SignatureScheme :: ECDSA_NISTP256_SHA256 , & signature :: ECDSA_P256_SHA256_ASN1_SIGNING ,) { return Ok (ecdsa_p256) ; } if let Ok (ecdsa_p384) = Self :: new (der , SignatureScheme :: ECDSA_NISTP384_SHA384 , & signature :: ECDSA_P384_SHA384_ASN1_SIGNING ,) { return Ok (ecdsa_p384) ; } Err (Error :: General ("failed to parse ECDSA private key as PKCS#8 or SEC1" . into () ,)) } }
};
}
