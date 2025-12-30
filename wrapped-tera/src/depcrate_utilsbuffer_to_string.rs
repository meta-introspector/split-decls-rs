// Generated macro for buffer_to_string (function)
macro_rules! Depcrate_utilsbuffer_to_string {
() => {
// Module: crate::utils
// Provides: {"buffer_to_string"}
// Dependencies: {}
pub (crate) fn buffer_to_string < F > (context : F , buffer : Vec < u8 >) -> Result < String , Error > where F : FnOnce () -> String , { String :: from_utf8 (buffer) . map_err (| error | Error :: utf8_conversion_error (error , context ())) }
};
}
