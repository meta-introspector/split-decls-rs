// Generated macro for impl_39 (impl)
macro_rules! Depcrate_rollingimpl_39 {
() => {
// Module: crate::rolling
// Provides: {"impl_39"}
// Dependencies: {}
impl io :: Write for RollingFileAppender { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let now = self . now () ; let writer = self . writer . get_mut () ; if let Some (current_time) = self . state . should_rollover (now) { let _did_cas = self . state . advance_date (now , current_time) ; debug_assert ! (_did_cas , "if we have &mut access to the appender, no other thread can have advanced the timestamp...") ; self . state . refresh_writer (now , writer) ; } writer . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . writer . get_mut () . flush () } }
};
}
