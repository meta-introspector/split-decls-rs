// Generated macro for other_362 (other)
macro_rules! Depcrate_import_exportother_362 {
() => {
// Module: crate::import_export
// Provides: {"other_362"}
// Dependencies: {}
extern "C" { # [cfg (target_os = "macos")] pub fn SecItemImport (importedData : CFDataRef , fileNameOrExtension : CFStringRef , inputFormat : * mut SecExternalFormat , itemType : * mut SecExternalItemType , flags : SecItemImportExportFlags , keyParams : * const SecItemImportExportKeyParameters , importKeychain : SecKeychainRef , outItems : * mut CFArrayRef ,) -> OSStatus ; # [cfg (target_os = "macos")] pub fn SecItemExport (secItemOrArray : CFTypeRef , outputFormat : SecExternalFormat , flags : SecItemImportExportFlags , keyParams : * const SecItemImportExportKeyParameters , exportedData : * mut CFDataRef ,) -> OSStatus ; pub static kSecImportExportPassphrase : CFStringRef ; # [cfg (target_os = "macos")] pub static kSecImportExportKeychain : CFStringRef ; # [cfg (target_os = "macos")] pub static kSecImportExportAccess : CFStringRef ; pub static kSecImportItemLabel : CFStringRef ; pub static kSecImportItemKeyID : CFStringRef ; pub static kSecImportItemTrust : CFStringRef ; pub static kSecImportItemCertChain : CFStringRef ; pub static kSecImportItemIdentity : CFStringRef ; pub fn SecPKCS12Import (pkcs12_data : CFDataRef , options : CFDictionaryRef , items : * mut CFArrayRef ,) -> OSStatus ; }
};
}
