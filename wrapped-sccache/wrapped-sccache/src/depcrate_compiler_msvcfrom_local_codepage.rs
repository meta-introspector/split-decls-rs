// Generated macro for from_local_codepage (function)
macro_rules! Depcrate_compiler_msvcfrom_local_codepage {
() => {
// Module: crate::compiler::msvc
// Provides: {"from_local_codepage"}
// Dependencies: {}
# [cfg (windows)] pub fn from_local_codepage (multi_byte_str : & [u8]) -> io :: Result < String > { use windows_sys :: Win32 :: Globalization :: { CP_OEMCP , MB_ERR_INVALID_CHARS , MultiByteToWideChar } ; let codepage = CP_OEMCP ; let flags = MB_ERR_INVALID_CHARS ; if multi_byte_str . is_empty () { return Ok (String :: new ()) ; } unsafe { let len = MultiByteToWideChar (codepage , flags , multi_byte_str . as_ptr () as _ , multi_byte_str . len () as i32 , std :: ptr :: null_mut () , 0 ,) ; if len > 0 { let mut wstr : Vec < u16 > = Vec :: with_capacity (len as usize) ; let len = MultiByteToWideChar (codepage , flags , multi_byte_str . as_ptr () as _ , multi_byte_str . len () as i32 , wstr . as_mut_ptr () as _ , len ,) ; if len > 0 { wstr . set_len (len as usize) ; return String :: from_utf16 (& wstr [0 .. (len as usize)]) . map_err (| e | io :: Error :: new (io :: ErrorKind :: InvalidInput , e)) ; } } Err (io :: Error :: last_os_error ()) } }
};
}
