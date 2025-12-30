// Generated macro for impl_40 (impl)
macro_rules! Depcrate_rollingimpl_40 {
() => {
// Module: crate::rolling
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a > tracing_subscriber :: fmt :: writer :: MakeWriter < 'a > for RollingFileAppender { type Writer = RollingWriter < 'a > ; fn make_writer (& 'a self) -> Self :: Writer { let now = self . now () ; if let Some (current_time) = self . state . should_rollover (now) { if self . state . advance_date (now , current_time) { self . state . refresh_writer (now , & mut self . writer . write ()) ; } } RollingWriter (self . writer . read ()) } }
};
}
