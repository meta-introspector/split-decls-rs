// Generated macro for multi_byte_to_wide_char (function)
macro_rules! Depcrate_utilmulti_byte_to_wide_char {
() => {
// Module: crate::util
// Provides: {"multi_byte_to_wide_char"}
// Dependencies: {}
# [cfg (windows)] # [doc = " Wrapper for MultiByteToWideChar."] # [doc = ""] # [doc = " See https://msdn.microsoft.com/en-us/library/windows/desktop/dd319072(v=vs.85).aspx"] # [doc = " for more details."] pub fn multi_byte_to_wide_char (codepage : u32 , flags : u32 , multi_byte_str : & [u8] ,) -> std :: io :: Result < Vec < u16 > > { use windows_sys :: Win32 :: Globalization :: MultiByteToWideChar ; if multi_byte_str . is_empty () { return Ok (vec ! []) ; } unsafe { let len = MultiByteToWideChar (codepage , flags , multi_byte_str . as_ptr () , multi_byte_str . len () as i32 , std :: ptr :: null_mut () , 0 ,) ; if len > 0 { let mut wstr : Vec < u16 > = Vec :: with_capacity (len as usize) ; let len = MultiByteToWideChar (codepage , flags , multi_byte_str . as_ptr () , multi_byte_str . len () as i32 , wstr . as_mut_ptr () , len ,) ; wstr . set_len (len as usize) ; if len > 0 { return Ok (wstr) ; } } Err (std :: io :: Error :: last_os_error ()) } }
};
}
