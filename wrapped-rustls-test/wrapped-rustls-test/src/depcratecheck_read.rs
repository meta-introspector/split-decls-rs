// Generated macro for check_read (function)
macro_rules! Depcratecheck_read {
() => {
// Module: crate
// Provides: {"check_read"}
// Dependencies: {}
# [doc = " Check `reader` has available exactly `bytes`"] pub fn check_read (reader : & mut dyn io :: Read , bytes : & [u8]) { let mut buf = vec ! [0u8 ; bytes . len () + 1] ; assert_eq ! (bytes . len () , reader . read (& mut buf) . unwrap ()) ; assert_eq ! (bytes , & buf [.. bytes . len ()]) ; }
};
}
