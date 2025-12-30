// Generated macro for write_list (function)
macro_rules! Depcratewrite_list {
() => {
// Module: crate
// Provides: {"write_list"}
// Dependencies: {}
fn write_list < T : Display > (list : impl IntoIterator < Item = T > , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_char ('[') ? ; let mut iter = list . into_iter () ; if let Some (item) = iter . next () { item . fmt (f) ? ; } for item in iter { f . write_str (", ") ? ; item . fmt (f) ? ; } f . write_char (']') }
};
}
