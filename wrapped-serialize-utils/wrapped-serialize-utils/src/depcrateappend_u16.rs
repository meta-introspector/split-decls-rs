// Generated macro for append_u16 (function)
macro_rules! Depcrateappend_u16 {
() => {
// Module: crate
// Provides: {"append_u16"}
// Dependencies: {}
pub fn append_u16 (buf : & mut Vec < u8 > , data : u16) { let start = buf . len () ; buf . resize (buf . len () + 2 , 0) ; let end = buf . len () ; buf [start .. end] . copy_from_slice (& data . to_le_bytes ()) ; }
};
}
