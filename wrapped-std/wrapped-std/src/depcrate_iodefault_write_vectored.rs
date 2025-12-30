// Generated macro for default_write_vectored (function)
macro_rules! Depcrate_iodefault_write_vectored {
() => {
// Module: crate::io
// Provides: {"default_write_vectored"}
// Dependencies: {}
pub (crate) fn default_write_vectored < F > (write : F , bufs : & [IoSlice < '_ >]) -> Result < usize > where F : FnOnce (& [u8]) -> Result < usize > , { let buf = bufs . iter () . find (| b | ! b . is_empty ()) . map_or (& [] [..] , | b | & * * b) ; write (buf) }
};
}
