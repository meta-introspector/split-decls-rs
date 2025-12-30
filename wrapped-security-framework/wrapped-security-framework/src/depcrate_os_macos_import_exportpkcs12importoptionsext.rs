// Generated macro for Pkcs12ImportOptionsExt (trait)
macro_rules! Depcrate_os_macos_import_exportPkcs12ImportOptionsExt {
() => {
// Module: crate::os::macos::import_export
// Provides: {"Pkcs12ImportOptionsExt"}
// Dependencies: {}
# [doc (hidden)] # [doc = " Obsolete. Use Pkcs12ImportOptions directly."] pub trait Pkcs12ImportOptionsExt { # [doc = " Specifies the keychain in which to import the identity."] # [doc = ""] # [doc = " If this is not called, the default keychain will be used."] fn keychain (& mut self , keychain : SecKeychain) -> & mut Self ; # [doc = " Specifies the access control to be associated with the identity."] fn access (& mut self , access : SecAccess) -> & mut Self ; }
};
}
