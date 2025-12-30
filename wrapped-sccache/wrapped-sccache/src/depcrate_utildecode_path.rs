// Generated macro for decode_path (function)
macro_rules! Depcrate_utildecode_path {
() => {
// Module: crate::util
// Provides: {"decode_path"}
// Dependencies: {}
# [cfg (windows)] pub fn decode_path (bytes : & [u8]) -> std :: io :: Result < PathBuf > { use windows_sys :: Win32 :: Globalization :: { CP_OEMCP , MB_ERR_INVALID_CHARS } ; let codepage = CP_OEMCP ; let flags = MB_ERR_INVALID_CHARS ; Ok (OsString :: from_wide (& multi_byte_to_wide_char (codepage , flags , bytes) ?) . into ()) }
};
}
