// Generated macro for delete_generic_password_options (function)
macro_rules! Depcrate_passwordsdelete_generic_password_options {
() => {
// Module: crate::passwords
// Provides: {"delete_generic_password_options"}
// Dependencies: {}
# [doc = " Delete the generic password keychain entry for the given service and account."] # [doc = " If none exists, fails with error code `errSecItemNotFound`."] # [doc = ""] # [doc = " See [`PasswordOptions`] and [`new_generic_password`](PasswordOptions::new_generic_password)."] # [doc = ""] # [doc = " ```rust"] # [doc = " use security_framework::passwords::{delete_generic_password_options, PasswordOptions};"] # [doc = " delete_generic_password_options(PasswordOptions::new_generic_password(\"service\", \"account\"));"] # [doc = " ```"] pub fn delete_generic_password_options (options : PasswordOptions) -> Result < () > { let params = options . to_dictionary () ; cvt (unsafe { SecItemDelete (params . as_concrete_TypeRef ()) }) }
};
}
