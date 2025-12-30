// Generated macro for impl_23 (impl)
macro_rules! Depcrate_errorimpl_23 {
() => {
// Module: crate::error
// Provides: {"impl_23"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [allow (unused_qualifications)] impl ErrorSink for alloc :: vec :: Vec < ParseError > { fn report_error (& mut self , error : ParseError) { self . push (error) ; } }
};
}
