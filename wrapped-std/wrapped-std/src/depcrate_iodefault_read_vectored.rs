// Generated macro for default_read_vectored (function)
macro_rules! Depcrate_iodefault_read_vectored {
() => {
// Module: crate::io
// Provides: {"default_read_vectored"}
// Dependencies: {}
pub (crate) fn default_read_vectored < F > (read : F , bufs : & mut [IoSliceMut < '_ >]) -> Result < usize > where F : FnOnce (& mut [u8]) -> Result < usize > , { let buf = bufs . iter_mut () . find (| b | ! b . is_empty ()) . map_or (& mut [] [..] , | b | & mut * * b) ; read (buf) }
};
}
