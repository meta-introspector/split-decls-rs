// Generated macro for FingerprintBytes (type)
macro_rules! DepcrateFingerprintBytes {
() => {
// Module: crate
// Provides: {"FingerprintBytes"}
// Dependencies: {}
# [doc = " Raw bytes of a SPKI fingerprint i.e. SHA-256 digest of"] # [doc = " `SubjectPublicKeyInfo`'s DER encoding."] # [doc = ""] # [doc = " See [RFC7469 § 2.1.1] for more information."] # [doc = ""] # [doc = " [RFC7469 § 2.1.1]: https://datatracker.ietf.org/doc/html/rfc7469#section-2.1.1"] # [cfg (feature = "fingerprint")] pub type FingerprintBytes = [u8 ; SIZE] ;
};
}
