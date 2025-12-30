// Generated macro for impl_259 (impl)
macro_rules! Depcrate_keyimpl_259 {
() => {
// Module: crate::key
// Provides: {"impl_259"}
// Dependencies: {}
# [allow (missing_docs)] impl KeyType { # [inline (always)] # [must_use] pub fn rsa () -> Self { unsafe { Self (kSecAttrKeyTypeRSA) } } # [cfg (target_os = "macos")] # [inline (always)] # [must_use] pub fn dsa () -> Self { unsafe { Self (kSecAttrKeyTypeDSA) } } # [cfg (target_os = "macos")] # [inline (always)] # [must_use] pub fn aes () -> Self { unsafe { Self (kSecAttrKeyTypeAES) } } # [cfg (target_os = "macos")] # [inline (always)] # [must_use] pub fn des () -> Self { unsafe { Self (kSecAttrKeyTypeDES) } } # [cfg (target_os = "macos")] # [inline (always)] # [must_use] pub fn triple_des () -> Self { unsafe { Self (kSecAttrKeyType3DES) } } # [cfg (target_os = "macos")] # [inline (always)] # [must_use] pub fn rc4 () -> Self { unsafe { Self (kSecAttrKeyTypeRC4) } } # [cfg (target_os = "macos")] # [inline (always)] # [must_use] pub fn cast () -> Self { unsafe { Self (kSecAttrKeyTypeCAST) } } # [inline (always)] # [must_use] pub fn ec () -> Self { use security_framework_sys :: item :: kSecAttrKeyTypeEC ; unsafe { Self (kSecAttrKeyTypeEC) } } # [inline (always)] # [must_use] pub fn ec_sec_prime_random () -> Self { use security_framework_sys :: item :: kSecAttrKeyTypeECSECPrimeRandom ; unsafe { Self (kSecAttrKeyTypeECSECPrimeRandom) } } pub (crate) fn to_str (self) -> CFString { unsafe { CFString :: wrap_under_get_rule (self . 0) } } }
};
}
