// Generated macro for set_internet_password (function)
macro_rules! Depcrate_passwordsset_internet_password {
() => {
// Module: crate::passwords
// Provides: {"set_internet_password"}
// Dependencies: {}
# [doc = " Set an internet password for the given endpoint parameters."] # [doc = " Creates or updates a keychain entry."] # [allow (clippy :: too_many_arguments)] pub fn set_internet_password (server : & str , security_domain : Option < & str > , account : & str , path : & str , port : Option < u16 > , protocol : SecProtocolType , authentication_type : SecAuthenticationType , password : & [u8] ,) -> Result < () > { let mut options = PasswordOptions :: new_internet_password (server , security_domain , account , path , port , protocol , authentication_type ,) ; set_password_internal (& mut options , password) }
};
}
