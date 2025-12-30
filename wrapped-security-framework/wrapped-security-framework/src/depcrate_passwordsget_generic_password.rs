// Generated macro for get_generic_password (function)
macro_rules! Depcrate_passwordsget_generic_password {
() => {
// Module: crate::passwords
// Provides: {"get_generic_password"}
// Dependencies: {}
# [doc = " Get the generic password for the given service and account.  If no matching"] # [doc = " keychain entry exists, fails with error code `errSecItemNotFound`."] # [doc (hidden)] pub fn get_generic_password (service : & str , account : & str) -> Result < Vec < u8 > > { generic_password (PasswordOptions :: new_generic_password (service , account)) }
};
}
