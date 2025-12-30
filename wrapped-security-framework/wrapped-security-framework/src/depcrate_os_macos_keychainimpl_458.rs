// Generated macro for impl_458 (impl)
macro_rules! Depcrate_os_macos_keychainimpl_458 {
() => {
// Module: crate::os::macos::keychain
// Provides: {"impl_458"}
// Dependencies: {}
impl KeychainSettings { # [doc = " Creates a new `KeychainSettings` with default settings."] # [inline] # [must_use] pub fn new () -> Self { Self (SecKeychainSettings { version : SEC_KEYCHAIN_SETTINGS_VERS1 , lockOnSleep : 0 , useLockInterval : 0 , lockInterval : i32 :: MAX as u32 , }) } # [doc = " If set, the keychain will automatically lock when the computer sleeps."] # [doc = ""] # [doc = " Defaults to `false`."] # [inline (always)] pub fn set_lock_on_sleep (& mut self , lock_on_sleep : bool) { self . 0 . lockOnSleep = Boolean :: from (lock_on_sleep) ; } # [doc = " Sets the interval of time in seconds after which the keychain is"] # [doc = " automatically locked."] # [doc = ""] # [doc = " Defaults to `None`."] pub fn set_lock_interval (& mut self , lock_interval : Option < u32 >) { if let Some (lock_interval) = lock_interval { self . 0 . useLockInterval = 1 ; self . 0 . lockInterval = lock_interval ; } else { self . 0 . useLockInterval = 0 ; self . 0 . lockInterval = i32 :: MAX as u32 ; } } }
};
}
