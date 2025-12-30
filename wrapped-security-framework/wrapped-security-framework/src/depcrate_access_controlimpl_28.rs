// Generated macro for impl_28 (impl)
macro_rules! Depcrate_access_controlimpl_28 {
() => {
// Module: crate::access_control
// Provides: {"impl_28"}
// Dependencies: {}
impl SecAccessControl { # [doc = " Create `AccessControl` object from flags"] pub fn create_with_flags (flags : CFOptionFlags) -> Result < Self > { Self :: create_with_protection (None , flags) } # [doc = " Create `AccessControl` object from a protection value and flags."] pub fn create_with_protection (protection : Option < ProtectionMode > , flags : CFOptionFlags) -> Result < Self > { let protection_val = protection . map (| v | { match v { ProtectionMode :: AccessibleWhenPasscodeSetThisDeviceOnly => unsafe { CFString :: wrap_under_get_rule (kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly) } , ProtectionMode :: AccessibleWhenUnlockedThisDeviceOnly => unsafe { CFString :: wrap_under_get_rule (kSecAttrAccessibleWhenUnlockedThisDeviceOnly) } , ProtectionMode :: AccessibleWhenUnlocked => unsafe { CFString :: wrap_under_get_rule (kSecAttrAccessibleWhenUnlocked) } , ProtectionMode :: AccessibleAfterFirstUnlockThisDeviceOnly => unsafe { CFString :: wrap_under_get_rule (kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly) } , ProtectionMode :: AccessibleAfterFirstUnlock => unsafe { CFString :: wrap_under_get_rule (kSecAttrAccessibleAfterFirstUnlock) } , } }) . unwrap_or_else (| | { unsafe { CFString :: wrap_under_get_rule (kSecAttrAccessibleWhenUnlocked) } }) ; unsafe { let access_control = SecAccessControlCreateWithFlags (kCFAllocatorDefault , protection_val . as_CFTypeRef () , flags , ptr :: null_mut () ,) ; if access_control . is_null () { Err (Error :: from_code (errSecParam)) } else { Ok (Self :: wrap_under_create_rule (access_control)) } } } }
};
}
