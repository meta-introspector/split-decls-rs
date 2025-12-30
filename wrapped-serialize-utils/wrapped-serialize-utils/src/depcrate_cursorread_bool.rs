// Generated macro for read_bool (function)
macro_rules! Depcrate_cursorread_bool {
() => {
// Module: crate::cursor
// Provides: {"read_bool"}
// Dependencies: {}
pub fn read_bool < T : AsRef < [u8] > > (cursor : & mut Cursor < T >) -> Result < bool , InstructionError > { let byte = read_u8 (cursor) ? ; match byte { 0 => Ok (false) , 1 => Ok (true) , _ => Err (InstructionError :: InvalidAccountData) , } }
};
}
