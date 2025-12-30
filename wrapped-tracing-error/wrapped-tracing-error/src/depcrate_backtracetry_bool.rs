// Generated macro for try_bool (macro)
macro_rules! Depcrate_backtracetry_bool {
() => {
// Module: crate::backtrace
// Provides: {"try_bool"}
// Dependencies: {}
macro_rules ! try_bool { ($ e : expr , $ dest : ident) => { { let ret = $ e . unwrap_or_else (| e | $ dest = Err (e)) ; if $ dest . is_err () { return false ; } ret } } ; }
};
}
