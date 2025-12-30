// Generated macro for read_i64 (function)
macro_rules! Depcrate_cursorread_i64 {
() => {
// Module: crate::cursor
// Provides: {"read_i64"}
// Dependencies: {}
pub fn read_i64 < T : AsRef < [u8] > > (cursor : & mut Cursor < T >) -> Result < i64 , InstructionError > { let mut buf = [0 ; 8] ; cursor . read_exact (& mut buf) . map_err (| _ | InstructionError :: InvalidAccountData) ? ; Ok (i64 :: from_le_bytes (buf)) }
};
}
