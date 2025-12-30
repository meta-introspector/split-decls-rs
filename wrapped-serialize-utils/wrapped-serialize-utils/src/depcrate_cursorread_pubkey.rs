// Generated macro for read_pubkey (function)
macro_rules! Depcrate_cursorread_pubkey {
() => {
// Module: crate::cursor
// Provides: {"read_pubkey"}
// Dependencies: {}
pub fn read_pubkey < T : AsRef < [u8] > > (cursor : & mut Cursor < T >) -> Result < Pubkey , InstructionError > { let mut buf = [0 ; 32] ; cursor . read_exact (& mut buf) . map_err (| _ | InstructionError :: InvalidAccountData) ? ; Ok (Pubkey :: from (buf)) }
};
}
