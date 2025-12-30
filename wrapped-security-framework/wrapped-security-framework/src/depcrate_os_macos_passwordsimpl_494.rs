// Generated macro for impl_494 (impl)
macro_rules! Depcrate_os_macos_passwordsimpl_494 {
() => {
// Module: crate::os::macos::passwords
// Provides: {"impl_494"}
// Dependencies: {}
impl SecKeychainItem { # [doc = " Modify keychain item in-place, replacing its password with the given one"] pub fn set_password (& mut self , password : & [u8]) -> Result < () > { unsafe { cvt (SecKeychainItemModifyAttributesAndData (self . as_concrete_TypeRef () , ptr :: null () , password . len () as u32 , password . as_ptr () . cast () ,)) ? ; } Ok (()) } # [doc = " Delete this item from its keychain"] # [inline] pub fn delete (self) { unsafe { SecKeychainItemDelete (self . as_concrete_TypeRef ()) ; } } }
};
}
