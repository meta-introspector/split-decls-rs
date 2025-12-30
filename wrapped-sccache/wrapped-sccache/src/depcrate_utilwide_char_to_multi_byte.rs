// Generated macro for wide_char_to_multi_byte (function)
macro_rules! Depcrate_utilwide_char_to_multi_byte {
() => {
// Module: crate::util
// Provides: {"wide_char_to_multi_byte"}
// Dependencies: {}
# [cfg (windows)] pub fn wide_char_to_multi_byte (wide_char_str : & [u16]) -> std :: io :: Result < Vec < u8 > > { use windows_sys :: Win32 :: Globalization :: { CP_OEMCP , WideCharToMultiByte } ; let codepage = CP_OEMCP ; let flags = 0 ; if wide_char_str . is_empty () { return Ok (Vec :: new ()) ; } unsafe { let len = WideCharToMultiByte (codepage , flags , wide_char_str . as_ptr () , wide_char_str . len () as i32 , std :: ptr :: null_mut () , 0 , std :: ptr :: null () , std :: ptr :: null_mut () ,) ; if len > 0 { let mut astr : Vec < u8 > = Vec :: with_capacity (len as usize) ; let len = WideCharToMultiByte (codepage , flags , wide_char_str . as_ptr () , wide_char_str . len () as i32 , astr . as_mut_ptr () as _ , len , std :: ptr :: null () , std :: ptr :: null_mut () ,) ; if len > 0 { astr . set_len (len as usize) ; if (len as usize) == astr . len () { return Ok (astr) ; } else { return Ok (astr [0 .. (len as usize)] . to_vec ()) ; } } } Err (std :: io :: Error :: last_os_error ()) } }
};
}
