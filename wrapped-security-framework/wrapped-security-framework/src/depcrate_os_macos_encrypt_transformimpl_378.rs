// Generated macro for impl_378 (impl)
macro_rules! Depcrate_os_macos_encrypt_transformimpl_378 {
() => {
// Module: crate::os::macos::encrypt_transform
// Provides: {"impl_378"}
// Dependencies: {}
# [allow (missing_docs)] impl Mode { # [inline (always)] # [must_use] pub fn none () -> Self { unsafe { Self (kSecModeNoneKey) } } # [inline (always)] # [must_use] pub fn ecb () -> Self { unsafe { Self (kSecModeECBKey) } } # [inline (always)] # [must_use] pub fn cbc () -> Self { unsafe { Self (kSecModeCBCKey) } } # [inline (always)] # [must_use] pub fn cfb () -> Self { unsafe { Self (kSecModeCFBKey) } } # [inline (always)] # [must_use] pub fn ofb () -> Self { unsafe { Self (kSecModeOFBKey) } } fn to_str (self) -> CFString { unsafe { CFString :: wrap_under_get_rule (self . 0) } } }
};
}
