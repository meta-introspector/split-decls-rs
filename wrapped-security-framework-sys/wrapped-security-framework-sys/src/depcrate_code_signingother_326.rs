// Generated macro for other_326 (other)
macro_rules! Depcrate_code_signingother_326 {
() => {
// Module: crate::code_signing
// Provides: {"other_326"}
// Dependencies: {}
extern "C" { pub static kSecGuestAttributeArchitecture : CFStringRef ; pub static kSecGuestAttributeAudit : CFStringRef ; pub static kSecGuestAttributeCanonical : CFStringRef ; pub static kSecGuestAttributeDynamicCode : CFStringRef ; pub static kSecGuestAttributeDynamicCodeInfoPlist : CFStringRef ; pub static kSecGuestAttributeHash : CFStringRef ; pub static kSecGuestAttributeMachPort : CFStringRef ; pub static kSecGuestAttributePid : CFStringRef ; pub static kSecGuestAttributeSubarchitecture : CFStringRef ; pub fn SecCodeGetTypeID () -> CFTypeID ; pub fn SecStaticCodeGetTypeID () -> CFTypeID ; pub fn SecRequirementGetTypeID () -> CFTypeID ; pub fn SecCodeCheckValidity (code : SecCodeRef , flags : SecCSFlags , requirement : SecRequirementRef ,) -> OSStatus ; pub fn SecCodeCopyGuestWithAttributes (host : SecCodeRef , attrs : CFDictionaryRef , flags : SecCSFlags , guest : * mut SecCodeRef ,) -> OSStatus ; pub fn SecCodeCopyPath (code : SecStaticCodeRef , flags : SecCSFlags , path : * mut CFURLRef ,) -> OSStatus ; pub fn SecCodeCopySelf (flags : SecCSFlags , out : * mut SecCodeRef) -> OSStatus ; pub fn SecRequirementCreateWithString (text : CFStringRef , flags : SecCSFlags , requirement : * mut SecRequirementRef ,) -> OSStatus ; pub fn SecStaticCodeCheckValidity (code : SecStaticCodeRef , flags : SecCSFlags , requirement : SecRequirementRef ,) -> OSStatus ; pub fn SecStaticCodeCreateWithPath (path : CFURLRef , flags : SecCSFlags , code : * mut SecStaticCodeRef ,) -> OSStatus ; }
};
}
