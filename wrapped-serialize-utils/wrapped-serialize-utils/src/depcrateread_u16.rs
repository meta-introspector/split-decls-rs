// Generated macro for read_u16 (function)
macro_rules! Depcrateread_u16 {
() => {
// Module: crate
// Provides: {"read_u16"}
// Dependencies: {}
pub fn read_u16 (current : & mut usize , data : & [u8]) -> Result < u16 , SanitizeError > { if data . len () < * current + 2 { return Err (SanitizeError :: IndexOutOfBounds) ; } let mut fixed_data = [0u8 ; 2] ; fixed_data . copy_from_slice (& data [* current .. * current + 2]) ; let e = u16 :: from_le_bytes (fixed_data) ; * current += 2 ; Ok (e) }
};
}
