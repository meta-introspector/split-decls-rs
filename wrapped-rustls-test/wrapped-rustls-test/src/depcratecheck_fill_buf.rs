// Generated macro for check_fill_buf (function)
macro_rules! Depcratecheck_fill_buf {
() => {
// Module: crate
// Provides: {"check_fill_buf"}
// Dependencies: {}
# [doc = " Check `reader` has available exactly `bytes`"] pub fn check_fill_buf (reader : & mut dyn io :: BufRead , bytes : & [u8]) { let b = reader . fill_buf () . unwrap () ; assert_eq ! (b , bytes) ; let len = b . len () ; reader . consume (len) ; }
};
}
