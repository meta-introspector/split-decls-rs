// Generated macro for impl_1297 (impl)
macro_rules! Depcrate_windows_systemimpl_1297 {
() => {
// Module: crate::windows::system
// Provides: {"impl_1297"}
// Dependencies: {}
impl RegKey { unsafe fn open (hkey : HKEY , path : & [u16]) -> Option < Self > { let mut new_hkey = Default :: default () ; if unsafe { RegOpenKeyExW (hkey , PCWSTR :: from_raw (path . as_ptr ()) , Some (0) , KEY_READ , & mut new_hkey ,) } . is_err () { return None ; } Some (Self (new_hkey)) } unsafe fn get_value (& self , field_name : & [u16] , buf : & mut [u8] , buf_len : & mut u32 ,) -> windows :: core :: Result < () > { let mut buf_type = REG_NONE ; unsafe { RegQueryValueExW (self . 0 , PCWSTR :: from_raw (field_name . as_ptr ()) , None , Some (& mut buf_type) , Some (buf . as_mut_ptr ()) , Some (buf_len) ,) } . ok () } }
};
}
