// Generated macro for impl_376 (impl)
macro_rules! Depcrate_os_macos_encrypt_transformimpl_376 {
() => {
// Module: crate::os::macos::encrypt_transform
// Provides: {"impl_376"}
// Dependencies: {}
impl Padding { # [doc = " Do not pad."] # [inline (always)] # [must_use] pub fn none () -> Self { unsafe { Self (kSecPaddingNoneKey) } } # [doc = " Use PKCS#1 padding."] # [inline (always)] # [must_use] pub fn pkcs1 () -> Self { unsafe { Self (kSecPaddingPKCS1Key) } } # [doc = " Use PKCS#5 padding."] # [inline (always)] # [must_use] pub fn pkcs5 () -> Self { unsafe { Self (kSecPaddingPKCS5Key) } } # [doc = " Use PKCS#7 padding."] # [inline (always)] # [must_use] pub fn pkcs7 () -> Self { unsafe { Self (kSecPaddingPKCS7Key) } } # [doc = " Use OAEP padding."] # [inline (always)] # [must_use] pub fn oaep () -> Self { unsafe { Self (kSecPaddingOAEPKey) } } # [inline] fn to_str (self) -> CFString { unsafe { CFString :: wrap_under_get_rule (self . 0) } } }
};
}
