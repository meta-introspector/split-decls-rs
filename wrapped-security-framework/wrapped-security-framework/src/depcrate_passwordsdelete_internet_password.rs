// Generated macro for delete_internet_password (function)
macro_rules! Depcrate_passwordsdelete_internet_password {
() => {
// Module: crate::passwords
// Provides: {"delete_internet_password"}
// Dependencies: {}
# [doc = " Delete the internet password for the given endpoint parameters."] # [doc = " If none exists, fails with error code `errSecItemNotFound`."] pub fn delete_internet_password (server : & str , security_domain : Option < & str > , account : & str , path : & str , port : Option < u16 > , protocol : SecProtocolType , authentication_type : SecAuthenticationType ,) -> Result < () > { let options = PasswordOptions :: new_internet_password (server , security_domain , account , path , port , protocol , authentication_type ,) ; let params = options . to_dictionary () ; cvt (unsafe { SecItemDelete (params . as_concrete_TypeRef ()) }) }
};
}
