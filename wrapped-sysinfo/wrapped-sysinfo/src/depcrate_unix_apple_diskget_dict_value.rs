// Generated macro for get_dict_value (function)
macro_rules! Depcrate_unix_apple_diskget_dict_value {
() => {
// Module: crate::unix::apple::disk
// Provides: {"get_dict_value"}
// Dependencies: {}
unsafe fn get_dict_value < T , F : FnOnce (* const c_void) -> Option < T > > (dict : & CFDictionary , key : Option < & CFString > , callback : F ,) -> Option < T > { let mut value = std :: ptr :: null () ; let key : * const CFString = key . map (| key | key as * const CFString) . unwrap_or (ptr :: null ()) ; if unsafe { dict . value_if_present (key . cast () , & mut value) } { callback (value) } else { None } }
};
}
