// Generated macro for error (macro)
macro_rules! Depcrate_errorerror {
() => {
// Module: crate::error
// Provides: {"error"}
// Dependencies: {}
macro_rules ! error { ($ fmt : literal $ (,) ?) => { Error (format ! ($ fmt)) } ; ($ fmt : literal , $ ($ arg : tt) *) => { Error (format ! ($ fmt , $ ($ arg) *)) } ; }
};
}
