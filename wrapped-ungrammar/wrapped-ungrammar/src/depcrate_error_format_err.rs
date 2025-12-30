// Generated macro for _format_err (macro)
macro_rules! Depcrate_error_format_err {
() => {
// Module: crate::error
// Provides: {"_format_err"}
// Dependencies: {}
macro_rules ! _format_err { ($ ($ tt : tt) *) => { $ crate :: error :: Error { message : format ! ($ ($ tt) *) , location : None , } } ; }
};
}
