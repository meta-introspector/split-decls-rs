// Generated macro for read_u8 (function)
macro_rules! Depcrateread_u8 {
() => {
// Module: crate
// Provides: {"read_u8"}
// Dependencies: {}
pub fn read_u8 (current : & mut usize , data : & [u8]) -> Result < u8 , SanitizeError > { if data . len () < * current + 1 { return Err (SanitizeError :: IndexOutOfBounds) ; } let e = data [* current] ; * current += 1 ; Ok (e) }
};
}
