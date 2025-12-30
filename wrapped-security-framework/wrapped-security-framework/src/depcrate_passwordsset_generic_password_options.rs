// Generated macro for set_generic_password_options (function)
macro_rules! Depcrate_passwordsset_generic_password_options {
() => {
// Module: crate::passwords
// Provides: {"set_generic_password_options"}
// Dependencies: {}
# [doc = " Set a generic password using the given password options."] # [doc = " Creates or updates a keychain entry."] pub fn set_generic_password_options (password : & [u8] , mut options : PasswordOptions) -> Result < () > { set_password_internal (& mut options , password) }
};
}
