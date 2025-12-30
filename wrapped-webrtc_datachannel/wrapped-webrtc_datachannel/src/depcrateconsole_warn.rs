// Generated macro for console_warn (macro)
macro_rules! Depcrateconsole_warn {
() => {
// Module: crate
// Provides: {"console_warn"}
// Dependencies: {}
macro_rules ! console_warn { ($ ($ t : tt) *) => (warn (& format_args ! ($ ($ t) *) . to_string ())) }
};
}
