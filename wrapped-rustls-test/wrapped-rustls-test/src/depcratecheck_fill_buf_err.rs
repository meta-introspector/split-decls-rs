// Generated macro for check_fill_buf_err (function)
macro_rules! Depcratecheck_fill_buf_err {
() => {
// Module: crate
// Provides: {"check_fill_buf_err"}
// Dependencies: {}
# [doc = " Check `reader` yields only an error of kind `err_kind`"] pub fn check_fill_buf_err (reader : & mut dyn io :: BufRead , err_kind : io :: ErrorKind) { let err = reader . fill_buf () . unwrap_err () ; assert ! (matches ! (err , err if err . kind () == err_kind)) }
};
}
