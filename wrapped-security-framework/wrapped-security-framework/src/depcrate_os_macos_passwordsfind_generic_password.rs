// Generated macro for find_generic_password (function)
macro_rules! Depcrate_os_macos_passwordsfind_generic_password {
() => {
// Module: crate::os::macos::passwords
// Provides: {"find_generic_password"}
// Dependencies: {}
# [doc = " Find a generic password."] # [doc = ""] # [doc = " The underlying system supports passwords with 0 values, so this"] # [doc = " returns a vector of bytes rather than a string."] # [doc = ""] # [doc = " * `keychains` is an array of keychains to search or None to search the default keychain."] # [doc = " * `service` is the name of the service to search for."] # [doc = " * `account` is the name of the account to search for."] pub fn find_generic_password (keychains : Option < & [SecKeychain] > , service : & str , account : & str ,) -> Result < (SecKeychainItemPassword , SecKeychainItem) > { let keychains_or_none = keychains . map (CFArray :: from_CFTypes) ; let keychains_or_null = match & keychains_or_none { None => ptr :: null () , Some (keychains) => keychains . as_CFTypeRef () , } ; let mut data_len = 0 ; let mut data = ptr :: null_mut () ; let mut item = ptr :: null_mut () ; unsafe { cvt (SecKeychainFindGenericPassword (keychains_or_null , service . len () as u32 , service . as_ptr () . cast () , account . len () as u32 , account . as_ptr () . cast () , & mut data_len , & mut data , & mut item ,)) ? ; Ok ((SecKeychainItemPassword { data : data as * const _ , data_len : data_len as usize , } , SecKeychainItem :: wrap_under_create_rule (item) ,)) } }
};
}
