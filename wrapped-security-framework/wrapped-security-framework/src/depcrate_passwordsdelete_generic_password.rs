// Generated macro for delete_generic_password (function)
macro_rules! Depcrate_passwordsdelete_generic_password {
() => {
// Module: crate::passwords
// Provides: {"delete_generic_password"}
// Dependencies: {}
# [doc = " Delete the generic password keychain entry for the given service and account."] # [doc = " If none exists, fails with error code `errSecItemNotFound`."] pub fn delete_generic_password (service : & str , account : & str) -> Result < () > { let options = PasswordOptions :: new_generic_password (service , account) ; delete_generic_password_options (options) }
};
}
