// Generated macro for impl_461 (impl)
macro_rules! Depcrate_os_macos_keychainimpl_461 {
() => {
// Module: crate::os::macos::keychain
// Provides: {"impl_461"}
// Dependencies: {}
# [cfg (target_os = "macos")] impl Drop for KeychainUserInteractionLock { # [inline (always)] fn drop (& mut self) { unsafe { SecKeychainSetUserInteractionAllowed (1u8) } ; } }
};
}
