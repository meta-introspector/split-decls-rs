// Generated macro for impl_358 (impl)
macro_rules! Depcrate_os_macos_digest_transformimpl_358 {
() => {
// Module: crate::os::macos::digest_transform
// Provides: {"impl_358"}
// Dependencies: {}
# [allow (missing_docs)] impl DigestType { # [inline (always)] # [must_use] pub fn hmac_md5 () -> Self { unsafe { Self (kSecDigestHMACMD5) } } # [inline (always)] # [must_use] pub fn hmac_sha1 () -> Self { unsafe { Self (kSecDigestHMACSHA1) } } # [inline (always)] # [must_use] pub fn hmac_sha2 () -> Self { unsafe { Self (kSecDigestHMACSHA2) } } # [inline (always)] # [must_use] pub fn md2 () -> Self { unsafe { Self (kSecDigestMD2) } } # [inline (always)] # [must_use] pub fn md4 () -> Self { unsafe { Self (kSecDigestMD4) } } # [inline (always)] # [must_use] pub fn md5 () -> Self { unsafe { Self (kSecDigestMD5) } } # [inline (always)] # [must_use] pub fn sha1 () -> Self { unsafe { Self (kSecDigestSHA1) } } # [inline (always)] # [must_use] pub fn sha2 () -> Self { unsafe { Self (kSecDigestSHA2) } } # [inline (always)] fn to_type (self) -> CFTypeRef { self . 0 as CFTypeRef } }
};
}
