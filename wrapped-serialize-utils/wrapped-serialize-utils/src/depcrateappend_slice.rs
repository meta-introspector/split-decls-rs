// Generated macro for append_slice (function)
macro_rules! Depcrateappend_slice {
() => {
// Module: crate
// Provides: {"append_slice"}
// Dependencies: {}
pub fn append_slice (buf : & mut Vec < u8 > , data : & [u8]) { let start = buf . len () ; buf . resize (buf . len () + data . len () , 0) ; let end = buf . len () ; buf [start .. end] . copy_from_slice (data) ; }
};
}
