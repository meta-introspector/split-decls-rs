// Generated macro for get_internet_password (function)
macro_rules! Depcrate_passwordsget_internet_password {
() => {
// Module: crate::passwords
// Provides: {"get_internet_password"}
// Dependencies: {}
# [doc = " Get the internet password for the given endpoint parameters.  If no matching"] # [doc = " keychain entry exists, fails with error code `errSecItemNotFound`."] pub fn get_internet_password (server : & str , security_domain : Option < & str > , account : & str , path : & str , port : Option < u16 > , protocol : SecProtocolType , authentication_type : SecAuthenticationType ,) -> Result < Vec < u8 > > { let mut options = PasswordOptions :: new_internet_password (server , security_domain , account , path , port , protocol , authentication_type ,) ; unsafe { options . push_query (kSecReturnData , CFBoolean :: from (true)) ; } let params = options . to_dictionary () ; let mut ret : CFTypeRef = std :: ptr :: null () ; cvt (unsafe { SecItemCopyMatching (params . as_concrete_TypeRef () , & mut ret) }) ? ; get_password_and_release (ret) }
};
}
