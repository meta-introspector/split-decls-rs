// Generated macro for get_int_value (function)
macro_rules! Depcrate_unix_apple_diskget_int_value {
() => {
// Module: crate::unix::apple::disk
// Provides: {"get_int_value"}
// Dependencies: {}
pub (super) unsafe fn get_int_value (dict : & CFDictionary , key : Option < & CFString >) -> Option < u64 > { unsafe { get_dict_value (dict , key , | v | { let v = & * v . cast :: < CFNumber > () ; Some (v . as_i64 () ? as u64) }) } }
};
}
