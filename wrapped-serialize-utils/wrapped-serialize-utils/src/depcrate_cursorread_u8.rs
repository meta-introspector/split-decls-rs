// Generated macro for read_u8 (function)
macro_rules! Depcrate_cursorread_u8 {
() => {
// Module: crate::cursor
// Provides: {"read_u8"}
// Dependencies: {}
pub fn read_u8 < T : AsRef < [u8] > > (cursor : & mut Cursor < T >) -> Result < u8 , InstructionError > { let mut buf = [0 ; 1] ; cursor . read_exact (& mut buf) . map_err (| _ | InstructionError :: InvalidAccountData) ? ; Ok (buf [0]) }
};
}
