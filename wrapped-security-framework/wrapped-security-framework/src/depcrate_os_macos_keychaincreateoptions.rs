// Generated macro for CreateOptions (struct)
macro_rules! Depcrate_os_macos_keychainCreateOptions {
() => {
// Module: crate::os::macos::keychain
// Provides: {"CreateOptions"}
// Dependencies: {}
# [doc = " A builder type to create new keychains."] # [derive (Default)] pub struct CreateOptions { password : Option < String > , prompt_user : bool , access : Option < SecAccess > , }
};
}
