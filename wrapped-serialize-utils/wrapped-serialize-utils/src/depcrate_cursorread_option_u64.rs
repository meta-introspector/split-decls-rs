// Generated macro for read_option_u64 (function)
macro_rules! Depcrate_cursorread_option_u64 {
() => {
// Module: crate::cursor
// Provides: {"read_option_u64"}
// Dependencies: {}
pub fn read_option_u64 < T : AsRef < [u8] > > (cursor : & mut Cursor < T > ,) -> Result < Option < u64 > , InstructionError > { let variant = read_u8 (cursor) ? ; match variant { 0 => Ok (None) , 1 => read_u64 (cursor) . map (Some) , _ => Err (InstructionError :: InvalidAccountData) , } }
};
}
