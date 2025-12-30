// Generated macro for read_u16 (function)
macro_rules! Depcrate_cursorread_u16 {
() => {
// Module: crate::cursor
// Provides: {"read_u16"}
// Dependencies: {}
pub fn read_u16 < T : AsRef < [u8] > > (cursor : & mut Cursor < T >) -> Result < u16 , InstructionError > { let mut buf = [0 ; 2] ; cursor . read_exact (& mut buf) . map_err (| _ | InstructionError :: InvalidAccountData) ? ; Ok (u16 :: from_le_bytes (buf)) }
};
}
