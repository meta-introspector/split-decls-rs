// Generated macro for get_bool_value (function)
macro_rules! Depcrate_unix_apple_diskget_bool_value {
() => {
// Module: crate::unix::apple::disk
// Provides: {"get_bool_value"}
// Dependencies: {}
unsafe fn get_bool_value (dict : & CFDictionary , key : Option < & CFString >) -> Option < bool > { unsafe { get_dict_value (dict , key , | v | { let v = & * v . cast :: < CFBoolean > () ; Some (v . as_bool ()) }) } }
};
}
