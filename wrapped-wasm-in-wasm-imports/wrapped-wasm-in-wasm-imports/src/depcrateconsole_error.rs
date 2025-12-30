// Generated macro for console_error (macro)
macro_rules! Depcrateconsole_error {
() => {
// Module: crate
// Provides: {"console_error"}
// Dependencies: {}
macro_rules ! console_error { ($ ($ t : tt) *) => (error (& format_args ! ($ ($ t) *) . to_string ())) }
};
}
