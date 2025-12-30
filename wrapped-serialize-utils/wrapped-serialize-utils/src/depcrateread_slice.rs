// Generated macro for read_slice (function)
macro_rules! Depcrateread_slice {
() => {
// Module: crate
// Provides: {"read_slice"}
// Dependencies: {}
pub fn read_slice (current : & mut usize , data : & [u8] , data_len : usize ,) -> Result < Vec < u8 > , SanitizeError > { if data . len () < * current + data_len { return Err (SanitizeError :: IndexOutOfBounds) ; } let e = data [* current .. * current + data_len] . to_vec () ; * current += data_len ; Ok (e) }
};
}
