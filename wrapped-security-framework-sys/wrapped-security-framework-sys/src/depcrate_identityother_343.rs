// Generated macro for other_343 (other)
macro_rules! Depcrate_identityother_343 {
() => {
// Module: crate::identity
// Provides: {"other_343"}
// Dependencies: {}
extern "C" { pub fn SecIdentityGetTypeID () -> CFTypeID ; pub fn SecIdentityCopyCertificate (identity : SecIdentityRef , certificate_ref : * mut SecCertificateRef ,) -> OSStatus ; pub fn SecIdentityCopyPrivateKey (identity : SecIdentityRef , key_ref : * mut SecKeyRef) -> OSStatus ; # [cfg (target_os = "macos")] pub fn SecIdentityCreateWithCertificate (keychain_or_Array : CFTypeRef , certificate_ref : SecCertificateRef , identity_ref : * mut SecIdentityRef ,) -> OSStatus ; }
};
}
