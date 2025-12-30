// Generated macro for SecItemImportExportKeyParameters (struct)
macro_rules! Depcrate_import_exportSecItemImportExportKeyParameters {
() => {
// Module: crate::import_export
// Provides: {"SecItemImportExportKeyParameters"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone)] # [cfg (target_os = "macos")] pub struct SecItemImportExportKeyParameters { pub version : c_uint , pub flags : SecKeyImportExportFlags , pub passphrase : CFTypeRef , pub alertTitle : CFStringRef , pub alertPrompt : CFStringRef , pub accessRef : SecAccessRef , pub keyUsage : CFArrayRef , pub keyAttributes : CFArrayRef , }
};
}
