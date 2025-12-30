// Generated macro for impl_493 (impl)
macro_rules! Depcrate_os_macos_passwordsimpl_493 {
() => {
// Module: crate::os::macos::passwords
// Provides: {"impl_493"}
// Dependencies: {}
impl Drop for SecKeychainItemPassword { # [inline] fn drop (& mut self) { unsafe { SecKeychainItemFreeContent (ptr :: null_mut () , self . data as * mut _) ; } } }
};
}
