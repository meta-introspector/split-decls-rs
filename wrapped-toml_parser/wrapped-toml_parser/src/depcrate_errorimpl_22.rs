// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorimpl_22 {
() => {
// Module: crate::error
// Provides: {"impl_22"}
// Dependencies: {}
impl ErrorSink for Option < ParseError > { fn report_error (& mut self , error : ParseError) { self . get_or_insert (error) ; } }
};
}
