// Generated macro for impl_207 (impl)
macro_rules! Depcrate_itemimpl_207 {
() => {
// Module: crate::item
// Provides: {"impl_207"}
// Dependencies: {}
impl ItemClass { # [doc = " Look for `SecKeychainItem`s corresponding to generic passwords."] # [inline (always)] # [must_use] pub fn generic_password () -> Self { unsafe { Self (kSecClassGenericPassword) } } # [doc = " Look for `SecKeychainItem`s corresponding to internet passwords."] # [inline (always)] # [must_use] pub fn internet_password () -> Self { unsafe { Self (kSecClassInternetPassword) } } # [doc = " Look for `SecCertificate`s."] # [inline (always)] # [must_use] pub fn certificate () -> Self { unsafe { Self (kSecClassCertificate) } } # [doc = " Look for `SecKey`s."] # [inline (always)] # [must_use] pub fn key () -> Self { unsafe { Self (kSecClassKey) } } # [doc = " Look for `SecIdentity`s."] # [inline (always)] # [must_use] pub fn identity () -> Self { unsafe { Self (kSecClassIdentity) } } }
};
}
