// Generated macro for default_read_buf (function)
macro_rules! Depcrate_iodefault_read_buf {
() => {
// Module: crate::io
// Provides: {"default_read_buf"}
// Dependencies: {}
pub (crate) fn default_read_buf < F > (read : F , mut cursor : BorrowedCursor < '_ >) -> Result < () > where F : FnOnce (& mut [u8]) -> Result < usize > , { let n = read (cursor . ensure_init () . init_mut ()) ? ; cursor . advance (n) ; Ok (()) }
};
}
