// Generated macro for SecItems (struct)
macro_rules! Depcrate_os_macos_import_exportSecItems {
() => {
// Module: crate::os::macos::import_export
// Provides: {"SecItems"}
// Dependencies: {}
# [doc = " A type which holds items imported from serialized data."] # [doc = ""] # [doc = " Pass a reference to `ImportOptions::items`."] # [derive (Default)] pub struct SecItems { # [doc = " Imported certificates."] pub certificates : Vec < SecCertificate > , # [doc = " Imported identities."] pub identities : Vec < SecIdentity > , # [doc = " Imported keys."] pub keys : Vec < SecKey > , }
};
}
