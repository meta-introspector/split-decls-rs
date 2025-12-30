// Generated macro for SecIdentityExt (trait)
macro_rules! Depcrate_os_macos_identitySecIdentityExt {
() => {
// Module: crate::os::macos::identity
// Provides: {"SecIdentityExt"}
// Dependencies: {}
# [doc = " An extension trait adding OSX specific functionality to `SecIdentity`."] pub trait SecIdentityExt { # [doc = " Creates an identity corresponding to a certificate, looking in the"] # [doc = " provided keychains for the corresponding private key."] # [doc = ""] # [doc = " To search the default keychains, use an empty slice for `keychains`."] # [doc = ""] # [doc = " <https://developer.apple.com/documentation/security/1401160-secidentitycreatewithcertificate>"] fn with_certificate (keychains : & [SecKeychain] , certificate : & SecCertificate ,) -> Result < SecIdentity > ; }
};
}
