// Generated macro for read_u32 (function)
macro_rules! Depcrate_cursorread_u32 {
() => {
// Module: crate::cursor
// Provides: {"read_u32"}
// Dependencies: {}
pub fn read_u32 < T : AsRef < [u8] > > (cursor : & mut Cursor < T >) -> Result < u32 , InstructionError > { let mut buf = [0 ; 4] ; cursor . read_exact (& mut buf) . map_err (| _ | InstructionError :: InvalidAccountData) ? ; Ok (u32 :: from_le_bytes (buf)) }
};
}
