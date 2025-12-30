// Generated macro for get_str_value (function)
macro_rules! Depcrate_unix_apple_diskget_str_value {
() => {
// Module: crate::unix::apple::disk
// Provides: {"get_str_value"}
// Dependencies: {}
pub (super) unsafe fn get_str_value (dict : & CFDictionary , key : Option < & CFString >) -> Option < String > { unsafe { get_dict_value (dict , key , | v | { let v = & * v . cast :: < CFString > () ; Some (v . to_string ()) }) } }
};
}
