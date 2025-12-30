// Generated macro for check_read_err (function)
macro_rules! Depcratecheck_read_err {
() => {
// Module: crate
// Provides: {"check_read_err"}
// Dependencies: {}
# [doc = " Check `reader` yields only an error of kind `err_kind`"] pub fn check_read_err (reader : & mut dyn io :: Read , err_kind : io :: ErrorKind) { let mut buf = vec ! [0u8 ; 1] ; let err = reader . read (& mut buf) . unwrap_err () ; assert ! (matches ! (err , err if err . kind () == err_kind)) }
};
}
