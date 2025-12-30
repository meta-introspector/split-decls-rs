// Generated macro for get_reg_value_u32 (function)
macro_rules! Depcrate_windows_systemget_reg_value_u32 {
() => {
// Module: crate::windows::system
// Provides: {"get_reg_value_u32"}
// Dependencies: {}
pub (crate) fn get_reg_value_u32 (hkey : HKEY , path : & str , field_name : & str) -> Option < [u8 ; 4] > { let c_path = utf16_str (path) ; let c_field_name = utf16_str (field_name) ; unsafe { let new_key = RegKey :: open (hkey , & c_path) ? ; let mut buf_len : u32 = 4 ; let mut buf = [0u8 ; 4] ; new_key . get_value (& c_field_name , & mut buf , & mut buf_len) . map (| _ | buf) . ok () } }
};
}
