// Generated macro for impl_95 (impl)
macro_rules! Depcrate_spanimpl_95 {
() => {
// Module: crate::span
// Provides: {"impl_95"}
// Dependencies: {}
impl Drop for Span { # [inline (always)] fn drop (& mut self) { if let Some (Inner { ref id , ref subscriber , }) = self . inner { subscriber . try_close (id . clone ()) ; } if_log_enabled ! { crate :: Level :: TRACE , { if let Some (meta) = self . meta { self . log (LIFECYCLE_LOG_TARGET , log :: Level :: Trace , format_args ! ("-- {};" , meta . name ()) ,) ; } } } } }
};
}
