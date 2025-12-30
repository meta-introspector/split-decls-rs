// Generated macro for _bail (macro)
macro_rules! Depcrate_error_bail {
() => {
// Module: crate::error
// Provides: {"_bail"}
// Dependencies: {}
macro_rules ! _bail { ($ ($ tt : tt) *) => { return Err ($ crate :: error :: format_err ! ($ ($ tt) *)) } ; }
};
}
