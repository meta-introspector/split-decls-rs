// Generated macro for generic_password (function)
macro_rules! Depcrate_passwordsgeneric_password {
() => {
// Module: crate::passwords
// Provides: {"generic_password"}
// Dependencies: {}
# [doc = " Get the generic password for the given service and account.  If no matching"] # [doc = " keychain entry exists, fails with error code `errSecItemNotFound`."] # [doc = ""] # [doc = " See [`PasswordOptions`] and [`new_generic_password`](PasswordOptions::new_generic_password)."] # [doc = ""] # [doc = " ```rust"] # [doc = " use security_framework::passwords::{generic_password, PasswordOptions};"] # [doc = " generic_password(PasswordOptions::new_generic_password(\"service\", \"account\"));"] # [doc = " ```"] pub fn generic_password (mut options : PasswordOptions) -> Result < Vec < u8 > > { unsafe { options . push_query (kSecReturnData , CFBoolean :: from (true)) ; } let params = options . to_dictionary () ; let mut ret : CFTypeRef = std :: ptr :: null () ; cvt (unsafe { SecItemCopyMatching (params . as_concrete_TypeRef () , & mut ret) }) ? ; get_password_and_release (ret) }
};
}
