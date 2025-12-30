// Generated macro for read_u64 (function)
macro_rules! Depcrate_cursorread_u64 {
() => {
// Module: crate::cursor
// Provides: {"read_u64"}
// Dependencies: {}
pub fn read_u64 < T : AsRef < [u8] > > (cursor : & mut Cursor < T >) -> Result < u64 , InstructionError > { let mut buf = [0 ; 8] ; cursor . read_exact (& mut buf) . map_err (| _ | InstructionError :: InvalidAccountData) ? ; Ok (u64 :: from_le_bytes (buf)) }
};
}
