// Generated macro for get_reg_string_value (function)
macro_rules! Depcrate_windows_systemget_reg_string_value {
() => {
// Module: crate::windows::system
// Provides: {"get_reg_string_value"}
// Dependencies: {}
pub (crate) fn get_reg_string_value (hkey : HKEY , path : & str , field_name : & str) -> Option < String > { let c_path = utf16_str (path) ; let c_field_name = utf16_str (field_name) ; unsafe { let new_key = RegKey :: open (hkey , & c_path) ? ; let mut buf_len : u32 = 2048 ; let mut buf : Vec < u8 > = Vec :: with_capacity (buf_len as usize) ; loop { match new_key . get_value (& c_field_name , & mut buf , & mut buf_len) { Ok (()) => break , Err (err) if err . code () == Foundation :: ERROR_MORE_DATA . to_hresult () => { buf . set_len (buf . capacity ()) ; buf . reserve (buf_len as _) ; } _ => return None , } } buf . set_len (buf_len as _) ; let words = std :: slice :: from_raw_parts (buf . as_ptr () as * const u16 , buf . len () / 2) ; let mut s = String :: from_utf16_lossy (words) ; while s . ends_with ('\u{0}') { s . pop () ; } Some (s) } }
};
}
