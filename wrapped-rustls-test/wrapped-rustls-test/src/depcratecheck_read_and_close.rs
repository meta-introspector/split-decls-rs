// Generated macro for check_read_and_close (function)
macro_rules! Depcratecheck_read_and_close {
() => {
// Module: crate
// Provides: {"check_read_and_close"}
// Dependencies: {}
# [doc = " Check `reader has available exactly `bytes`, followed by EOF"] pub fn check_read_and_close (reader : & mut dyn io :: Read , expect : & [u8]) { check_read (reader , expect) ; assert ! (matches ! (reader . read (& mut [0u8 ; 5]) , Ok (0))) ; }
};
}
