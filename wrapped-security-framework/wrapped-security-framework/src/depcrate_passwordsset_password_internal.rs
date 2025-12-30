// Generated macro for set_password_internal (function)
macro_rules! Depcrate_passwordsset_password_internal {
() => {
// Module: crate::passwords
// Provides: {"set_password_internal"}
// Dependencies: {}
fn set_password_internal (options : & mut PasswordOptions , password : & [u8]) -> Result < () > { # [allow (deprecated)] let query_without_password = options . query . len () ; unsafe { options . push_query (kSecValueData , CFData :: from_buffer (password)) ; } let params = options . to_dictionary () ; let mut ret = std :: ptr :: null () ; let status = unsafe { SecItemAdd (params . as_concrete_TypeRef () , & mut ret) } ; if status == errSecDuplicateItem { # [allow (deprecated)] let (query , pass) = options . query . split_at (query_without_password) ; let params = CFDictionary :: from_CFType_pairs (query) ; let update = CFDictionary :: from_CFType_pairs (pass) ; cvt (unsafe { SecItemUpdate (params . as_concrete_TypeRef () , update . as_concrete_TypeRef ()) }) } else { cvt (status) } }
};
}
