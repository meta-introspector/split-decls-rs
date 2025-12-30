// Generated macro for console_log (macro)
macro_rules! Depcrateconsole_log {
() => {
// Module: crate
// Provides: {"console_log"}
// Dependencies: {}
# [doc = " Helper macro which acts like `println!` only routes to `console.log`"] # [doc = " instead."] # [macro_export] macro_rules ! console_log { ($ ($ arg : tt) *) => ($ crate :: __rt :: console_log (& format_args ! ($ ($ arg) *))) }
};
}
