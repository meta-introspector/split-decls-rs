// Generated macro for Pkcs12ImportOptions (struct)
macro_rules! Depcrate_import_exportPkcs12ImportOptions {
() => {
// Module: crate::import_export
// Provides: {"Pkcs12ImportOptions"}
// Dependencies: {}
# [doc = " A builder type to import an identity from PKCS#12 formatted data."] # [derive (Default)] pub struct Pkcs12ImportOptions { passphrase : Option < CFString > , # [cfg (target_os = "macos")] keychain : Option < SecKeychain > , # [cfg (target_os = "macos")] access : Option < SecAccess > , }
};
}
