// Generated macro for set_generic_password (function)
macro_rules! Depcrate_passwordsset_generic_password {
() => {
// Module: crate::passwords
// Provides: {"set_generic_password"}
// Dependencies: {}
# [doc = " Set a generic password for the given service and account."] # [doc = " Creates or updates a keychain entry."] pub fn set_generic_password (service : & str , account : & str , password : & [u8]) -> Result < () > { let mut options = PasswordOptions :: new_generic_password (service , account) ; set_password_internal (& mut options , password) }
};
}
