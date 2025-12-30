// Generated macro for impl_1912 (impl)
macro_rules! Depcrate_enumsimpl_1912 {
() => {
// Module: crate::enums
// Provides: {"impl_1912"}
// Dependencies: {}
impl SignatureScheme { pub (crate) fn algorithm (& self) -> SignatureAlgorithm { match * self { Self :: RSA_PKCS1_SHA1 | Self :: RSA_PKCS1_SHA256 | Self :: RSA_PKCS1_SHA384 | Self :: RSA_PKCS1_SHA512 | Self :: RSA_PSS_SHA256 | Self :: RSA_PSS_SHA384 | Self :: RSA_PSS_SHA512 => SignatureAlgorithm :: RSA , Self :: ECDSA_SHA1_Legacy | Self :: ECDSA_NISTP256_SHA256 | Self :: ECDSA_NISTP384_SHA384 | Self :: ECDSA_NISTP521_SHA512 => SignatureAlgorithm :: ECDSA , Self :: ED25519 => SignatureAlgorithm :: ED25519 , Self :: ED448 => SignatureAlgorithm :: ED448 , _ => SignatureAlgorithm :: Unknown (0) , } } # [doc = " Whether a particular `SignatureScheme` is allowed for TLS protocol signatures"] # [doc = " in TLS1.3."] # [doc = ""] # [doc = " This prevents (eg) RSA_PKCS1_SHA256 being offered or accepted, even if our"] # [doc = " verifier supports it for other protocol versions."] # [doc = ""] # [doc = " See RFC8446 s4.2.3: <https://datatracker.ietf.org/doc/html/rfc8446#section-4.2.3>"] # [doc = ""] # [doc = " This is a denylist so that newly-allocated `SignatureScheme`s values are"] # [doc = " allowed in TLS1.3 by default."] pub (crate) fn supported_in_tls13 (& self) -> bool { let [hash , sign] = self . to_array () ; match HashAlgorithm :: from (hash) { HashAlgorithm :: NONE | HashAlgorithm :: MD5 | HashAlgorithm :: SHA1 | HashAlgorithm :: SHA224 => return false , _ => () , } ; ! matches ! (SignatureAlgorithm :: from (sign) , SignatureAlgorithm :: Anonymous | SignatureAlgorithm :: RSA | SignatureAlgorithm :: DSA) } }
};
}
