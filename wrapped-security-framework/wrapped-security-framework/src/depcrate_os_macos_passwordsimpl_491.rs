// Generated macro for impl_491 (impl)
macro_rules! Depcrate_os_macos_passwordsimpl_491 {
() => {
// Module: crate::os::macos::passwords
// Provides: {"impl_491"}
// Dependencies: {}
impl AsRef < [u8] > for SecKeychainItemPassword { # [inline] fn as_ref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self . data , self . data_len) } } }
};
}
