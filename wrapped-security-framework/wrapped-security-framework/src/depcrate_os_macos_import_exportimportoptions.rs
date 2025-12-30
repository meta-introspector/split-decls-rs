// Generated macro for ImportOptions (struct)
macro_rules! Depcrate_os_macos_import_exportImportOptions {
() => {
// Module: crate::os::macos::import_export
// Provides: {"ImportOptions"}
// Dependencies: {}
# [doc = " A builder type to import Security Framework types from serialized formats."] # [derive (Default)] pub struct ImportOptions < 'a > { filename : Option < CFString > , passphrase : Option < CFType > , secure_passphrase : bool , no_access_control : bool , alert_title : Option < CFString > , alert_prompt : Option < CFString > , items : Option < & 'a mut SecItems > , keychain : Option < SecKeychain > , }
};
}
