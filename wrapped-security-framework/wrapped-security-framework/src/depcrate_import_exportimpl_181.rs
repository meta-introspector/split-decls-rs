// Generated macro for impl_181 (impl)
macro_rules! Depcrate_import_exportimpl_181 {
() => {
// Module: crate::import_export
// Provides: {"impl_181"}
// Dependencies: {}
# [cfg (target_os = "macos")] impl Pkcs12ImportOptions { # [doc = " Specifies macOS keychain in which to import the identity."] # [doc = ""] # [doc = " If this is not called, the default keychain will be used."] # [inline (always)] pub fn keychain (& mut self , keychain : SecKeychain) -> & mut Self { self . keychain = Some (keychain) ; self } # [doc = " Specifies the access control to be associated with the identity. macOS only."] # [inline (always)] pub fn access (& mut self , access : SecAccess) -> & mut Self { self . access = Some (access) ; self } }
};
}
