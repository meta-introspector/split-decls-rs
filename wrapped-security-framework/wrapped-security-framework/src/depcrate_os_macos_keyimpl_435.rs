// Generated macro for impl_435 (impl)
macro_rules! Depcrate_os_macos_keyimpl_435 {
() => {
// Module: crate::os::macos::key
// Provides: {"impl_435"}
// Dependencies: {}
impl SecKeyExt for SecKey { fn from_data (key_type : KeyType , key_data : & CFData) -> Result < Self , CFError > { unsafe { let key = CFString :: wrap_under_get_rule (kSecAttrKeyType) ; let dict = CFDictionary :: from_CFType_pairs (& [(key , key_type . to_str ())]) ; let mut err = ptr :: null_mut () ; let key = SecKeyCreateFromData (dict . as_concrete_TypeRef () , key_data . as_concrete_TypeRef () , & mut err ,) ; if key . is_null () { Err (CFError :: wrap_under_create_rule (err)) } else { Ok (Self :: wrap_under_create_rule (key)) } } } }
};
}
