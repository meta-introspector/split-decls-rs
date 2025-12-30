// Generated macro for write_binary (function)
macro_rules! Depcratewrite_binary {
() => {
// Module: crate
// Provides: {"write_binary"}
// Dependencies: {}
fn write_binary (bytes : & [u8] , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_char ('[') ? ; let mut iter = bytes . iter () . copied () ; if let Some (value) = iter . next () { value . fmt (f) ? ; } for value in iter { f . write_str (", ") ? ; value . fmt (f) ? ; } f . write_char (']') }
};
}
