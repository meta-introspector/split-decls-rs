// Generated macro for append_u8 (function)
macro_rules! Depcrateappend_u8 {
() => {
// Module: crate
// Provides: {"append_u8"}
// Dependencies: {}
pub fn append_u8 (buf : & mut Vec < u8 > , data : u8) { let start = buf . len () ; buf . resize (buf . len () + 1 , 0) ; buf [start] = data ; }
};
}
